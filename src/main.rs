mod snakegame;
mod view;
mod snake;
mod common;

use macroquad::prelude as mq;
use snakegame::*;
use view::*;

#[macroquad::main(window_conf)]
async fn main() {
    mq::rand::srand(49152);
    let mut game = SnakeGame::new();
    let mut view = View::new(33, 25, 24);

    loop {
        match &game.state {
            GameState::Start => {
                game.reset(&mut view);
            },
            GameState::GetReady => {
                game.update_view(&mut view);
                game.draw(&view);
                if view.state == ViewState::Ready {
                    game.state = GameState::Playing;
                }
            },
            GameState::Playing => {
                game.handle_input();
                game.update_data(&view);
                game.check_collision(&view);
                game.update_view(&mut view);
                game.draw(&view);
            },
            GameState::GameOver => { 
                game.handle_input();
                game.update_view(&mut view);
                game.draw(&view);
            },
            _ => { }
        }

        mq::next_frame().await;
    }
}

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: "Snake".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}





