use std::collections::LinkedList;
use macroquad::prelude::*;
use crate::gamestate::*;
use crate::gamescene::*;
use crate::apple::*;
use crate::common::*;

//=================================================================================================
//    Snake
//=================================================================================================
struct SnakeFrame;

impl SnakeFrame {
    const TAIL: f32 = 0.0; 
    const BODY: f32 = 1.0;
    const HEAD: f32 = 2.0;
    const CORNER_NW: f32 = 3.0;
    const CORNER_NE: f32 = 4.0;
    const CORNER_SW: f32 = 5.0;
    const CORNER_SE: f32 = 6.0;
    const TONGUE_1: f32 = 7.0;
    const TONGUE_2: f32 = 8.0;
    const DEAD_HEAD: f32 = 9.0;
}

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
    id: u8,
    pub color: Color,
    initial_pos: Vec2,
    parts: LinkedList<SnakePart>,
    removed_part: Option<SnakePart>,
    tongue_anim_flag: bool,
    new_dir: Direction,
    speed: f32,
    timer: Timer,
    alive: bool,
    bound: Rect
}

impl Snake {
    const INITIAL_SPEED: f32 = 3.0;
    const MAX_SPEED: f32 = 10.0;
    pub const STUN_INTERVAL: f32 = 0.8;
    const NORMAL_DYING_INTERVAL: f32 = 0.2;
    const FAST_DYING_INTERVAL: f32 = 0.05;

    pub fn new(id: u8, color: Color, initial_pos: Vec2, bound: Rect) -> Snake {
        let parts: LinkedList<SnakePart> = LinkedList::new(); 
        let removed_part = None;
        let tongue_anim_flag = false;
        let new_dir = Direction::Up;
        let speed = 0.0; 
        let timer = Timer::new(0.0);
        let alive = true;

        let mut snake = Snake { id, color, initial_pos, parts, removed_part, tongue_anim_flag, 
                                new_dir, speed, timer, alive, bound };
        snake.reset();
        snake
    }

    pub fn reset(&mut self) {
        let x = self.initial_pos.x;
        let y = self.initial_pos.y;

        self.parts.clear();
        self.parts.push_back(SnakePart::new(Vec2::new(x, y), Direction::Up) );
        self.parts.push_back(SnakePart::new(Vec2::new(x, y + 1.0), Direction::Up) );
        self.parts.push_back(SnakePart::new(Vec2::new(x, y + 2.0), Direction::Up) );
        self.new_dir = Direction::Up;
        self.speed = Snake::INITIAL_SPEED; 
        self.timer = Timer::new(1.0 / self.speed); 
        self.alive = true;
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn set_direction(&mut self, dir: Direction) -> bool {
        let mut dir_changed = false;
        let current_dir = self.direction();
        let invalid_dir = opposite_dir(current_dir); 

        if dir != current_dir && dir != invalid_dir {
            self.new_dir = dir;
            dir_changed = true;
        }

        dir_changed
    }

    pub fn kill_self(&mut self) {
        self.alive = false;
        let length: f32  = self.parts.len() as f32;

        let interval: f32;
        if length < 10.0 {
            interval = Snake::NORMAL_DYING_INTERVAL;
        } else {
            interval = Snake::FAST_DYING_INTERVAL;
        }

        self.timer.set(interval);
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn length(&self) -> u32 {
        self.parts.len() as u32
    }

    pub fn _speed(&self) -> f32 {
        self.speed
    }

    pub fn position(&self) -> Vec2 {
        match self.parts.front() {
            Some(part) => part.pos, 
            None => Vec2::new(-1.0, -1.0), 
        }
    }

    pub fn new_position(&self) -> Vec2 {
        let cur_pos = self.position();
        let offset = dir_to_vec2(self.new_dir);
        Vec2::new(cur_pos.x + offset.x, cur_pos.y + offset.y)
    }

    pub fn is_position_taken(&self, pos: &Vec2) -> bool {
        for part in &self.parts {
            if *pos == part.pos {
                return true;
            }
        }
        return false;
    }

    pub fn eat_apples(&mut self, apples: &mut Vec<Apple>) -> bool {
        let mut result = false;
        let mut i = 0;
        
        for apple in apples.iter() {
            if self.position() == apple.pos {  // Run into an apple
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

    pub fn check_collision(&self, gamestate: &GameState, opponent_id: i32) -> bool {
        let mut collided = false;
        let pos = self.new_position();

        if !self.bound.contains(pos) {  // Collision with walls
            println!("Player {} crashed into the wall!", self.id + 1);
            collided = true;
        } else if self.is_collided_with_snake(&pos, &self) {  // Collision with tail
            println!("Player {} crashed into yourself!", self.id + 1);
            collided = true;
        } else {
            let opponent_optional = gamestate.player_by_id(opponent_id);
            match opponent_optional {  // Collision with the opponent
                Some(opponent) => if self.is_collided_with_snake(&pos, opponent) {  
                    println!("Player {} crashed into the opponent!", self.id + 1);
                    collided = true;
                },
                None => { }
            }
        }
        
        collided
    }

    pub fn check_update_time(&mut self) -> bool {
        self.timer.update()
    }

    pub fn update(&mut self) {
        if self.alive { 
            if self.alive {  // If it's still alive after collision check
                self.tongue_anim_flag = !self.tongue_anim_flag;
                let new_head = SnakePart::new(self.new_position(), self.new_dir);
                self.parts.push_front(new_head); 
                let removed_part = self.parts.pop_back().unwrap();
                self.removed_part = Some(removed_part);
            }
            self.timer.reset();  
        }
    }

    pub fn dying(&mut self) -> bool {  // Return false when finished dying
        if self.timer.update() && self.parts.len() != 0 {
            self.parts.pop_back().unwrap();
            self.timer.reset();
        }
        
        self.parts.len() != 0
    }

    pub fn draw_basic(&self, scene: &GameScene) {
        let mut is_head = true;
        let mut color = self.color;

        for part in &self.parts {
            if part.pos.y <= self.bound.bottom() {
                scene.draw_block(&part.pos, &color);
            }

            if is_head { 
                is_head = false;
                color.r -= 0.2;
                color.g -= 0.2;
                color.b -= 0.2;
            } 
        }
    }

    pub fn draw(&self, texture: &Texture2D, draw_tongue: bool, scene: &GameScene) {
        let length: u32 = self.parts.len() as u32;
        if length == 0 {
            return
        }

        let mut frame_index: f32;
        let mut rotation: f32;
        let mut prev_dir: Option<Direction> = None;
        let mut part_index: u32 = 0;
        let tail: u32 = length - 1;

        for part in &self.parts {
            match &mut prev_dir {
                Some(_dir) => { }, 
                None => prev_dir = Some(part.dir)
            }

            if part.dir == prev_dir.unwrap() || part_index == tail {
                if part_index == 0 {
                    if self.alive {
                        frame_index = SnakeFrame::HEAD;
                    } else {
                        frame_index = SnakeFrame::DEAD_HEAD;
                    }
                    rotation = self.rotation_from_direction(&part.dir);
                } else if part_index == tail {
                    frame_index = SnakeFrame::TAIL; 
                    rotation = self.rotation_from_direction(&prev_dir.unwrap());
                } else {
                    frame_index = SnakeFrame::BODY;
                    rotation = self.rotation_from_direction(&part.dir);
                }
            } else {  // Corners
                frame_index = self.corner_frame_index(&prev_dir.unwrap(), &part.dir);  
                rotation = 0.0;
                prev_dir = Some(part.dir);
            }

            if part.pos.y <= self.bound.bottom() {
                scene.draw_texture_atlas(texture, 16.0, frame_index, &part.pos, 
                                         &WHITE, rotation);
            }

            part_index += 1;
        }

        // Draw tongue
        if draw_tongue {
            frame_index = match self.tongue_anim_flag {
                false => SnakeFrame::TONGUE_1,
                true => SnakeFrame::TONGUE_2,
            };
            let cur_dir = self.direction(); 
            let tongue_pos = self.position() + dir_to_vec2(cur_dir); 
            let tongue_rotation = self.rotation_from_direction(&cur_dir);
            scene.draw_texture_atlas(texture, 16.0, frame_index, &tongue_pos, &WHITE, tongue_rotation);
        }
    }

//=================================================================================================    
//  Private methods (Snake)
//=================================================================================================    
    fn is_collided_with_snake(&self, pos: &Vec2, snake: &Snake) -> bool {
        let mut collided = false;

        for part in &snake.parts {
            if *pos == part.pos {
                collided = true;
                break
            }
        }

        collided
    }

    fn rotation_from_direction(&self, dir: &Direction) -> f32 {
        match dir {
            Direction::Up => 3.14 + 3.14 * 0.5,
            Direction::Right => 0.0, 
            Direction::Down => 3.14 * 0.5,
            Direction::Left => 3.14,
        }
    }

    fn corner_frame_index(&self, prev_dir: &Direction, dir: &Direction) 
            -> f32 {
        match prev_dir {
            Direction::Up => {
                if *dir == Direction::Left { SnakeFrame::CORNER_SW }
                else { SnakeFrame::CORNER_SE }
            },
            Direction::Down => {
                if *dir == Direction::Left { SnakeFrame::CORNER_NW }
                else { SnakeFrame::CORNER_NE }
            },
            Direction::Left => {
                if *dir == Direction::Up { SnakeFrame::CORNER_NE }
                else { SnakeFrame::CORNER_SE }
            },
            Direction::Right => {
                if *dir == Direction::Up { SnakeFrame::CORNER_NW }
                else { SnakeFrame::CORNER_SW }
            }
        }
    }

    fn direction(&self) -> Direction {
        self.parts.front().unwrap().dir
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

}


