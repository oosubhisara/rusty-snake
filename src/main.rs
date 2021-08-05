mod assets;
mod gamestate;
mod gamescene;
mod snake;
mod apple;
mod label;
mod common;

use macroquad::prelude::*;
use gamestate::*;

const WINDOW_WIDTH: f32 = 960.0;
const WINDOW_HEIGHT: f32 = 640.0;
const WIDTH: f32 = 23.0;
const HEIGHT: f32 = 20.0;
const GRID_SIZE: f32 = 32.0;

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(49152);

    println!("Screen size: {}x{}", screen_width(), screen_height());

    let mut game = GameState::new(WIDTH, HEIGHT, GRID_SIZE);
    game.load().await;
    game.start();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: (WINDOW_WIDTH) as i32,
        window_height: (WINDOW_HEIGHT) as i32, 
        window_resizable: false,
        ..Default::default()
    }
}





