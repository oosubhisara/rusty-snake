use macroquad::prelude::*;
use crate::snake::*;
use crate::gamescene::*;

//=================================================================================================
//    Apple
//=================================================================================================
pub struct Apple {
    pub pos: Vec2,
    pub alpha: f32
}

impl Apple {
    pub fn random_spawn(play_area: &Rect, players: &[Snake]) -> Apple {
        let pos: Vec2;

        loop {
            // Randomize position inside playing field excluding borders
            let x: f32 = (rand::rand() % play_area.w as u32) as f32;
            let y: f32 = (rand::rand() % play_area.h as u32) as f32;
            // +1 to start after left & top border
            pos = Vec2::new(x + play_area.x, y + play_area.y); 

            // Do not spawn on top of the snake
            for player in players {
                if player.is_position_taken(&pos) {
                    continue;
                }
            }

            break;
        }

        Apple { pos: pos, alpha: 0.25 }
    }

    pub fn update(&mut self) {
        self.alpha += 0.01;
        if self.alpha > 1.0 {
            self.alpha = 1.0;
        }
    }

    pub fn draw_basic(&mut self, scene: &GameScene) {
        let color = Color::new(1.0, 0.0, 0.0, self.alpha);
        scene.draw_circle(&self.pos, &color);
    }

    pub fn draw(&mut self, texture: &Texture2D, scene: &GameScene) {
        let color = Color::new(1.0, 1.0, 1.0, self.alpha);
        scene.draw_texture(texture, &self.pos, &color);
    }
}
