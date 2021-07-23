use macroquad::prelude::*;
use crate::common::*;

//=============================================================================
//    GameScene
//=============================================================================
pub struct GameScene {
    width: f32,
    height: f32,
    grid_size: f32,
    grid_alpha: f32,
    lines: Vec<Line>,
    timer: Timer,
}

impl GameScene {
    const GRID_FADE_SPEED: f32 = 0.02;

    pub fn new(width: f32, height: f32, grid_size: f32) 
            -> Self {
        let grid_alpha = 0.7;
        let lines = Vec::new(); 
        let timer = Timer::new(1.0 / 60.0); 
        let mut new_scene = Self { 
            width, height, grid_size, grid_alpha, 
            lines, timer
        };
        new_scene.reset();
        new_scene
    }

    pub fn reset(&mut self) {
        self.lines.clear();

        let mut alpha: f32 = 0.0;
        let alpha_diff = 0.05;

        for y in 0..self.height as u32 {
            let line = Line::new( 
                0.0, 
                y as f32 * self.grid_size,
                (self.width - 1.0) * self.grid_size,
                y as f32 * self.grid_size,
                alpha);
            self.lines.push(line);
            alpha -= alpha_diff; 
        }

        alpha = 0.0;
        for x in 0..self.width as u32 {
            let line = Line::new(
                x as f32 * self.grid_size,
                0.0, 
                x as f32 * self.grid_size,
                (self.height - 1.0) * self.grid_size,
                alpha);
            self.lines.push(line);
            alpha -= alpha_diff; 
        }

    }

    fn to_view_coord(&self, pos: &Vec2) -> Vec2 {
        Vec2::new(pos.x * self.grid_size as f32, 
                      pos.y * self.grid_size  as f32)
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn animate_grid(&mut self) -> bool {
        let mut finished_lines = 0;

        if self.timer.update() {
            for line in &mut self.lines {
                line.alpha += GameScene::GRID_FADE_SPEED;
                if line.alpha > self.grid_alpha {
                    line.alpha = self.grid_alpha; 
                    finished_lines += 1;
                }
            }
            self.timer.reset();
        }

        finished_lines != self.lines.len() 
    }

    pub fn draw(&self, texture: &Texture2D) {
        self.draw_grid();
        self.draw_texture_borders(texture);
    }

    pub fn draw_basic(&self) {
        self.draw_grid();
        self.draw_borders();
    }

    fn draw_grid(&self) {
        for line in &self.lines {
            let color = Color::new(0.663, 0.373, 0.263, line.alpha);
            draw_line(line.x1, line.y1, line.x2, line.y2, 1.0, color); 
        }
    }

    fn draw_borders(&self) {
        let color = BLUE;

        for x in 0..self.width as u32{
            self.draw_block(&Vec2::new(x as f32, 0.0), &color);
            self.draw_block(&Vec2::new(x as f32 , 
                            self.height as f32 - 1.0), &color);
        }

        for y in 0..self.height as u32 {
            self.draw_block(&Vec2::new(0.0, y as f32), &color);
            self.draw_block(&Vec2::new(self.width as f32 - 1.0, y as f32), 
                            &color);
        }
    }

    fn draw_texture_borders(&self, texture: &Texture2D) {
        let color = Color::new(1.0, 1.0, 1.0, 1.0);

        for x in 0..self.width as u32{
            self.draw_texture(texture, &Vec2::new(x as f32, 0.0), &color);
            self.draw_texture(texture, &Vec2::new(x as f32, 
                            self.height as f32 - 1.0), &color);
        }

        for y in 0..self.height as u32 {
            self.draw_texture(texture, &Vec2::new(0.0, y as f32), &color);
            self.draw_texture(texture, &Vec2::new(
                            self.width as f32 - 1.0, y as f32), &color);
        }
    }

    pub fn draw_block(&self, pos: &Vec2, color: &Color) {
        let draw_pos = self.to_view_coord(pos);
        draw_rectangle(draw_pos.x, draw_pos.y, 
                       self.grid_size as f32, self.grid_size as f32, 
                       *color);
    }

    pub fn draw_circle(&self, pos: &Vec2, color: &Color) {
        let draw_pos = self.to_view_coord(pos);
        draw_circle(draw_pos.x + self.grid_size / 2.0, 
                    draw_pos.y + self.grid_size / 2.0, 
                    (self.grid_size / 4.0) as f32,
                    *color);
    }

    pub fn draw_texture(&self, texture: &Texture2D, pos: &Vec2, 
                        color: &Color) {
        let draw_pos = self.to_view_coord(pos);
        let mut draw_params: DrawTextureParams = DrawTextureParams::default();
        draw_params.dest_size = Some(
            Vec2::new(self.grid_size, self.grid_size)
        );
        draw_texture_ex(*texture, draw_pos.x, draw_pos.y, *color, draw_params);
    }

    pub fn draw_texture_by_index(&self, texture: &Texture2D, src_size: f32,
                                 atlas_index: f32, pos: &Vec2, 
                                 color: &Color, rotation: f32) {
        let draw_pos = self.to_view_coord(pos);
        let mut draw_params: DrawTextureParams = DrawTextureParams::default();
        draw_params.source = Some(
            Rect::new((atlas_index * src_size) as f32, 0.0, src_size, src_size)
        ); 
        draw_params.dest_size = Some(
            Vec2::new(self.grid_size, self.grid_size)
        );
        draw_params.rotation = rotation;
        draw_texture_ex(*texture, draw_pos.x, draw_pos.y, *color, draw_params);
    }

}

