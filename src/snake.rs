use std::collections::LinkedList;
use macroquad::prelude as mq;
use crate::view::*;
use crate::common::*;

//=============================================================================
//    Snake
//=============================================================================
pub struct Snake {
    parts: LinkedList<Point>,
    direction: Point,
    speed: f32,
    timer: f32,
    alive: bool,
}

impl Snake {
    pub fn new() -> Snake {
        let mut parts: LinkedList<Point> = LinkedList::new(); 
        let direction = Direction::right();
        let speed = 0.0; 
        let timer = 0.0;
        let alive = true;

        Snake { parts, direction, speed, timer, alive }
    }

    pub fn reset(&mut self, x: i32, y: i32) {
        self.parts.clear();
        self.parts.push_back( Point { x: x, y: y } );
        self.parts.push_back( Point { x: x-1, y: y } );
        self.parts.push_back( Point { x: x-2, y: y } );
        self.direction = Direction::right();
        self.speed = 1.0 / 4.0;
        self.timer = 0.0;
        self.alive = true;
    }

    pub fn set_direction(&mut self, dir: &Point) {
        let invalid_dir = Direction::opposite(&self.direction); 

        if *dir != invalid_dir {
            self.direction.x = dir.x;
            self.direction.y = dir.y;
        }
   
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn get_position(&self) -> Point {
        match self.parts.front() {
            Some(pt) => Point { x: pt.x, y: pt.y },
            None => Point { x: -1, y: -1 }, 
        }
    }

    pub fn get_new_position(&self) -> Point {
        let current_position = self.get_position();
        Point { 
            x: current_position.x + self.direction.x,
            y: current_position.y + self.direction.y 
        }
    }

    pub fn covered_position(&self, position: &Point) -> bool {
        for part in &self.parts {
            if position == part {
                return true;
            }
        }
        return false;
    }

    pub fn check_collision(&mut self, view: &View) {
        let pos = self.get_new_position();

        if pos.x < 1 || pos.x > view.get_width() as i32 - 2 {
            self.alive = false;
        }
        if pos.y < 1 || pos.y > view.get_height()  as i32 - 2 {
            self.alive = false;
        }
        
        // check collision with tail
        for part in &self.parts {
            if &pos == part {
                self.alive = false;
                break
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.alive {
            return
        }

        self.timer += delta_time;
        if self.timer >= self.speed {
            self.timer = 0.0;

            // Update position of snake parts
            let new_position = self.get_new_position(); 
            self.parts.push_front(new_position);
            self.parts.pop_back();
        }
    }

    pub fn draw(&self, view: &View) {
        let mut head = true;
        let color1 = mq::Color::new(0.0, 1.0, 0.0, 1.0);
        let color2 = mq::Color::new(0.0, 0.7, 0.0, 1.0);

        for part in &self.parts {
            if head {
                head = false;
                view.draw_block(part, &color1);
            } else {
                view.draw_block(part, &color2);
            }
        }
    }
}


