mod assets;
mod gamestate;
mod gamescene;
mod snake;
mod apple;
mod common;

use macroquad::prelude::*;
use gamestate::*;

const WIDTH: f32 = 25.0;
const HEIGHT: f32 = 16.0;
const GRID_SIZE: f32 = 32.0;
const STATUS_HEIGHT: i32 = 88;

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(49152);

    println!("Screen size: {}x{}", screen_width(), screen_height());

    let mut game = GameState::new(WIDTH, HEIGHT, GRID_SIZE, 2);
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
        window_width: (WIDTH * GRID_SIZE) as i32,
        window_height: (HEIGHT * GRID_SIZE) as i32 + STATUS_HEIGHT, 
        window_resizable: false,
        ..Default::default()
    }
}





