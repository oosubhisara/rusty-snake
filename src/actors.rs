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
}

//=============================================================================
//    Snake
//=============================================================================
pub struct Snake {
    parts: LinkedList<Vec2>,
    removed_part: Option<Vec2>,
    dir: Direction, 
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
        let parts: LinkedList<Vec2> = LinkedList::new(); 
        let removed_part = None;
        let dir = Direction::NONE;
        let new_dir = Direction::NONE;
        let speed = 0.0; 
        let timer = Timer::new(0.0);
        let alive = true;
        let bound = Rect::new(0.0, 0.0, 0.0, 0.0);

        let mut snake = Snake { parts, removed_part, dir, new_dir, 
                            speed, timer, alive, bound };
        snake.reset(x, y);
        snake
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.parts.clear();
        self.parts.push_back( Vec2::new(x, y) );
        self.parts.push_back( Vec2::new(x - 1.0, y) );
        self.parts.push_back( Vec2::new(x - 2.0, y) );
        self.dir = Direction::RIGHT;
        self.new_dir = Direction::RIGHT;
        self.speed = Snake::INITIAL_SPEED; 
        self.timer = Timer::new(1.0 / self.speed); 
        self.alive = true;
    }

    pub fn set_bound(&mut self, rect: &Rect) {
        self.bound = Rect::new(rect.x, rect.y, rect.w, rect.h);
    }

    pub fn set_direction(&mut self, dir: Direction) -> bool {
        let mut dir_changed = false;
        let invalid_dir = get_opposite_dir(self.dir); 

        if dir != self.dir && dir != invalid_dir {
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
            Some(pt) => Vec2::new(pt.x, pt.y),
            None => Vec2::new(-1.0, -1.0), 
        }
    }

    pub fn get_new_position(&self) -> Vec2 {
        let cur_pos = self.get_position();
        let offset = dir_to_vec2(self.new_dir);
        Vec2::new(cur_pos.x + offset.x, cur_pos.y + offset.y)
    }

    pub fn is_position_overlapped(&self, pos: &Vec2) -> bool {
        for part in &self.parts {
            if pos == part {
                return true;
            }
        }
        return false;
    }

    fn restore_removed_path(&mut self) {
        match &self.removed_part {
            Some(p) => {
                let part = Vec2::new(p.x, p.y);
                self.parts.push_back(part);
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
            if &pos == part {
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
                self.dir = self.new_dir;  // Update direction
                let new_pos = self.get_new_position();  // Update position 
                self.parts.push_front(new_pos);  // Update parts
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

    pub fn draw(&self, game_scene: &GameScene) {
        let mut head = true;
        let head_color = Color::new(0.0, 1.0, 0.0, 1.0); 
        let body_color = Color::new(0.0, 0.7, 0.0, 1.0);

        for part in &self.parts {
            if head {
                head = false;
                game_scene.draw_block(part, &head_color);
            } else {
                game_scene.draw_block(part, &body_color);
            }
        }
    }
}


