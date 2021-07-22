use std::collections::LinkedList;
use macroquad::prelude::*;
use crate::gamescene::*;
use crate::common::*;

//=============================================================================
//    Apple
//=============================================================================
pub struct Apple {
    pub pos: Vec2,
    pub alpha: f32
}

impl Apple {
    pub fn new(pos: &Vec2) -> Apple {
        Apple { pos: *pos, alpha: 0.0 }
    }

    pub fn update(&mut self) {
        self.alpha += 0.01;
        if self.alpha > 1.0 {
            self.alpha = 1.0;
        }
    }

    pub fn draw(&mut self, scene: &GameScene) {
        let color = Color::new(1.0, 0.0, 0.0, self.alpha);
        scene.draw_circle(&self.pos, &color);
    }
}

//=============================================================================
//    Snake
//=============================================================================
#[derive(Clone)]
struct SnakePart {
    pos: Vec2,
    dir: Direction
}

impl SnakePart {
    fn new(pos: Vec2, dir: Direction) -> SnakePart {
        SnakePart { pos, dir }
    }
}

pub struct Snake {
    parts: LinkedList<SnakePart>,
    removed_part: Option<SnakePart>,
    new_dir: Direction,
    speed: f32,
    timer: Timer,
    alive: bool,
    bound: Rect
}

impl Snake {
    const INITIAL_SPEED: f32 = 5.0;
    const MAX_SPEED: f32 = 10.0;
    pub const STUN_INTERVAL: f32 = 0.5;
    const FAST_DYING_INTERVAL: f32 = 0.3;
    const SLOW_DYING_INTERVAL: f32 = 0.8;

    pub fn new(x: f32, y: f32) -> Snake {
        let parts: LinkedList<SnakePart> = LinkedList::new(); 
        let removed_part = None;
        let new_dir = Direction::Right;
        let speed = 0.0; 
        let timer = Timer::new(0.0);
        let alive = true;
        let bound = Rect::new(0.0, 0.0, 0.0, 0.0);

        let mut snake = Snake { parts, removed_part, new_dir, 
                                speed, timer, alive, bound };
        snake.reset(x, y);
        snake
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.parts.clear();
        self.parts.push_back(
            SnakePart::new(Vec2::new(x, y), Direction::Right) );
        self.parts.push_back( 
            SnakePart::new(Vec2::new(x - 1.0, y), Direction::Right) );
        self.parts.push_back( 
            SnakePart::new(Vec2::new(x - 2.0, y), Direction::Right) );
        self.new_dir = Direction::Right;
        self.speed = Snake::INITIAL_SPEED; 
        self.timer = Timer::new(1.0 / self.speed); 
        self.alive = true;
    }

    pub fn set_bound(&mut self, rect: &Rect) -> &Snake {
        self.bound = Rect::new(rect.x, rect.y, rect.w, rect.h);
        self 
    }

    pub fn set_direction(&mut self, dir: Direction) -> bool {
        let mut dir_changed = false;
        let current_dir = self.get_current_dir();
        let invalid_dir = get_opposite_dir(current_dir); 

        if dir != current_dir && dir != invalid_dir {
            self.new_dir = dir;
            dir_changed = true;
        }

        dir_changed
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn get_position(&self) -> Vec2 {
        match self.parts.front() {
            Some(part) => part.pos, 
            None => Vec2::new(-1.0, -1.0), 
        }
    }

    fn get_current_dir(&self) -> Direction {
        self.parts.front().unwrap().dir
    }

    pub fn get_new_position(&self) -> Vec2 {
        let cur_pos = self.get_position();
        let offset = dir_to_vec2(self.new_dir);
        Vec2::new(cur_pos.x + offset.x, cur_pos.y + offset.y)
    }

    pub fn is_position_overlapped(&self, pos: &Vec2) -> bool {
        for part in &self.parts {
            if *pos == part.pos {
                return true;
            }
        }
        return false;
    }

    fn restore_removed_path(&mut self) {
        match &mut self.removed_part {
            Some(part) => {
                let restored_part = part.clone();
                self.parts.push_back(restored_part);
                self.removed_part = None;
            },
            None => { }
        }
    }

    pub fn eat_apples(&mut self, apples: &mut Vec<Apple>) -> bool {
        let mut result = false;
        let mut i = 0;
        
        for apple in apples.iter() {
            if self.get_position() == apple.pos {  // Run into an apple
                result = true;
                apples.remove(i);
                self.restore_removed_path();  // Snake glows!
                self.speed += 0.1;
                if self.speed > Snake::MAX_SPEED {
                    self.speed = Snake::MAX_SPEED;
                }
                self.timer.set(1.0 / self.speed);
                println!("Speed: {}", self.speed);
                break;
            }

            i += 1;
        }

        result
    }

    pub fn check_collision(&mut self) {
        let pos = self.get_new_position();

        // Check bounds
        if !self.bound.contains(pos) {
            println!("You ran into the wall and died at ({},{})", 
                     pos.x, pos.y);
            self.dead();
        }

        // Check collision with tail
        for part in &self.parts {
            if pos == part.pos {
                println!("You ran into your tail and died at ({},{})", 
                         pos.x, pos.y);
                self.dead();
                break
            }
        }
    }

    fn dead(&mut self) {
        self.alive = false;
        let length: f32  = self.parts.len() as f32;

        let interval: f32;
        if length <= 5.0 {
            interval = Snake::FAST_DYING_INTERVAL;
        } else {
            interval = Snake::SLOW_DYING_INTERVAL;
        }

        self.timer.set(interval / length);
    }

    pub fn update(&mut self) -> bool {
        let mut updated = false;

        if self.alive && self.timer.update() {  
            // Check collision 
            self.check_collision();

            // Update 
            if self.alive {  // If it's still alive after collision check
                let new_head = SnakePart::new(
                    self.get_new_position(), self.new_dir);
                self.parts.push_front(new_head); 
                let removed_part = self.parts.pop_back().unwrap();
                self.removed_part = Some(removed_part);
            }
            self.timer.reset();  
            updated = true;
        }
        
        updated
    }

    pub fn dying(&mut self) -> bool {  // Return false when finished dying
        if self.timer.update() && self.parts.len() != 0 {
            self.parts.pop_back().unwrap();
            self.timer.reset();
        }
        
        self.parts.len() != 0
    }

    pub fn draw(&self, texture: &Texture2D, scene: &GameScene) {
        let mut part_index: u32 = 0;

        for part in &self.parts {
            let frame_index: f32 = 
                if part_index == 0 {
                    2.0
                } else if part_index == self.parts.len() as u32 - 1 {
                    0.0
                } else {
                    1.0
                };

            let rotation: f32 = match part.dir {
                Direction::Up => 3.14 + 3.14 * 0.5,
                Direction::Right => 0.0, 
                Direction::Down => 3.14 * 0.5,
                Direction::Left => 3.14,
            };

            scene.draw_texture_by_index(texture, 16.0, frame_index, 
                                        &part.pos, &WHITE, rotation);
            part_index += 1;
        }
    }
}


