use macroquad::prelude::*;

//=============================================================================
//    Timer
//=============================================================================
pub struct Timer {
    duration: f32, 
    counter: f32
}

impl Timer {
    pub fn new(duration: f32) -> Timer {
        let counter = 0.0;
        Timer { duration, counter }
    }

    pub fn set(&mut self, duration: f32) {
        self.duration = duration;
        self.counter = 0.0;
    }

    pub fn reset(&mut self) {
        self.counter = 0.0;
    }

    pub fn update(&mut self) -> bool {
        let mut alarm = false;
        self.counter += get_frame_time();

        if self.counter >= self.duration {
            alarm = true;
        }

        alarm
    }
}

//=============================================================================
//    Line
//=============================================================================
pub struct Line {
    pub x1: f32, 
    pub y1: f32,
    pub x2: f32, 
    pub y2: f32,
    pub alpha: f32,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, alpha: f32) -> Line {
        Line  { x1, y1, x2, y2, alpha }
    }
}

//=============================================================================
//    Direction
//=============================================================================
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up, Down, Left, Right 
}

pub fn dir_to_vec2(dir: Direction) -> Vec2 {
    match dir {
        Direction::Up => Vec2::new(0.0, -1.0), 
        Direction::Down => Vec2::new(0.0, 1.0), 
        Direction::Left => Vec2::new(-1.0, 0.0), 
        Direction::Right => Vec2::new(1.0, 0.0) 
    }
}

pub fn opposite_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up, 
        Direction::Left =>  Direction::Right,
        Direction::Right => Direction::Left,
    }
}


pub fn text_center_pos(text: &str, text_params: TextParams,
                       rect: &Rect) -> Vec2 {
    let dimension: TextDimensions = 
        measure_text(text, Some(text_params.font), text_params.font_size, 
                             1.0); 
    Vec2::new(rect.x + rect.w / 2.0 - dimension.width / 2.0,
              rect.y + rect.h / 2.0 - dimension.height / 2.0 
             + dimension.offset_y / 2.0)
}

pub fn draw_text_center(text: &str, text_params: TextParams, shadow_color: Color, offset: f32,
                        rect: &Rect) {
    let dimension: TextDimensions = 
        measure_text(text, Some(text_params.font), text_params.font_size, 
                             1.0); 
    let pos = Vec2::new(rect.x + rect.w / 2.0 - dimension.width / 2.0,
              rect.y + rect.h / 2.0 - dimension.height / 2.0 
             + dimension.offset_y / 2.0);
    let mut shadow_params = text_params.clone();
    shadow_params.color = shadow_color; 
    draw_text_ex(text, pos.x + offset, pos.y + offset, shadow_params);
    draw_text_ex(text, pos.x, pos.y, text_params);
}

