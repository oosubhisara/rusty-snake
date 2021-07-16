use macroquad::prelude as mq;
use crate::common::*;

//=============================================================================
//    View
//=============================================================================
#[derive(Eq, PartialEq)]
pub enum ViewState {
    AnimateGrid, Ready
}

pub struct View {
    width: u32,
    height: u32,
    grid_size: u32,
    grid_alpha: f32,
    lines: Vec<Line>,
    pub state: ViewState,
    timer: f32,
}

impl View {
    const GRID_FADE_SPEED: f32 = 0.05;

    pub fn new(width: u32, height: u32, grid_size: u32) 
            -> Self {
        let grid_alpha = 0.7;
        let lines = Vec::new(); 
        let state = ViewState::AnimateGrid;
        let timer = 0.0;
        let mut new_view = Self { 
            width, height, grid_size, grid_alpha, 
            lines, state, timer
        };
        new_view.reset();
        new_view
    }

    pub fn reset(&mut self) {
        self.state = ViewState::AnimateGrid;
        self.lines.clear();

        let mut alpha: f32 = 0.0;
        let alpha_diff = 0.05;

        for y in 0..self.height {
            let line = self.make_line(
                0, 
                (y * self.grid_size) as i32, 
                ((self.width - 1) * self.grid_size) as i32, 
                (y * self.grid_size) as i32, 
                alpha);
            self.lines.push(line);
            alpha -= alpha_diff; 
        }

        alpha = 0.0;
        for x in 0..self.width {
            let line = self.make_line(
                (x * self.grid_size) as i32, 
                0, 
                (x * self.grid_size) as i32, 
                ((self.height - 1) * self.grid_size) as i32, 
                alpha);
            self.lines.push(line);
            alpha -= alpha_diff; 
        }

    }

    fn make_line(&self, x1: i32, y1: i32, x2: i32, y2: i32, alpha: f32) 
            -> Line {
        Line { x1: x1 as f32, y1: y1 as f32, 
               x2: x2 as f32, y2: y2 as f32, 
               alpha: alpha }
    }

    fn to_view_coord(&self, position: &Point) -> Point {
        Point { x: position.x * self.grid_size as i32, 
                y: position.y * self.grid_size  as i32 }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn update(&mut self, delta_time: f32) {
        self.timer += delta_time;
        
        if self.state == ViewState::AnimateGrid {
            if self.timer > 1.0 / 60.0 {
                self.timer = 0.0;
                self.update_grid();
            }
        }
    }

    fn update_grid(&mut self) {
        let mut finished = 0;

        for line in &mut self.lines {
            line.alpha += View::GRID_FADE_SPEED;
            if line.alpha > self.grid_alpha {
                line.alpha = self.grid_alpha; 
                finished += 1;
            }
        }

        if finished == self.lines.len() {
            self.state = ViewState::Ready;
        }
    }

    pub fn draw(&self) {
        self.draw_grid();
        self.draw_borders();
    }

    fn draw_grid(&self) {
        let mut i = 0;
        for line in &self.lines {
            i += 1;
            let color = mq::Color::new(0.8, 0.5, 1.0, line.alpha);
            mq::draw_line(line.x1, line.y1, line.x2, line.y2, 1.0, color); 
        }
    }

    fn draw_borders(&self) {
        let color = mq::BLUE;

        for x in 0..self.width {
            self.draw_block(&Point {x: x as i32, y: 0}, &color);
            self.draw_block(&Point {x: x as i32, y: (self.height - 1) as i32}, 
                &color);
        }

        for y in 0..self.height {
            self.draw_block(&Point {x: 0, y: y as i32}, &color);
            self.draw_block(&Point {x: (self.width - 1) as i32, y: y as i32}, 
                &color);
        }
    }

    pub fn draw_block(&self, position: &Point, color: &mq::Color) {
        let pos = self.to_view_coord(position);
        mq::draw_rectangle(pos.x as f32, pos.y as f32, 
                           self.grid_size as f32, self.grid_size as f32, 
                           *color);
    }
}

