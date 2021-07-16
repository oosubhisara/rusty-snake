use macroquad::prelude as mq;
use crate::view::*;
use crate::snake::*;
use crate::common::*;

//=============================================================================
//    SnakeGame
//=============================================================================
#[derive(Eq, PartialEq)]
pub enum GameState {
    Start, GetReady, Playing, Dying, GameOver
}

pub struct SnakeGame {
    title: String,
    snake: Snake,
    apples: Vec<Point>,
    spawn_timer: f32,
    pub state: GameState,
}

impl SnakeGame {
    const SPAWN_TIME: f32 = 2.0;
    const MAX_APPLES: usize = 3;

    pub fn new() -> SnakeGame {
        let title = String::from("Snake Game");
        let snake = Snake::new();
        let apples = Vec::new();
        let spawn_timer = 0.0;
        let state = GameState::Start;

        SnakeGame { title, snake, apples, spawn_timer, state }
    }

    pub fn reset(&mut self, view: &mut View) {
        view.reset();
        self.snake.reset(3, 3);
        self.apples.clear();
        self.spawn_timer = 0.0;
        self.state = GameState::GetReady;
    }

    pub fn handle_input(&mut self) {
        match self.state {
            GameState::Playing => {
                if mq::is_key_down(mq::KeyCode::Up) {
                    self.snake.set_direction(&Direction::up());
                }
                else if mq::is_key_down(mq::KeyCode::Down) {
                    self.snake.set_direction(&Direction::down());
                }
                else if mq::is_key_down(mq::KeyCode::Left) {
                    self.snake.set_direction(&Direction::left());
                }
                else if mq::is_key_down(mq::KeyCode::Right) {
                    self.snake.set_direction(&Direction::right());
                }
            },
            GameState::GameOver => {
                if mq::is_key_down(mq::KeyCode::Enter) {
                    self.state = GameState::Start;
                }
            },
            _ => { }
        }

    }

    pub fn check_collision(&mut self, view: &View) {
        self.snake.check_collision(view);

        if !self.snake.is_alive() {
            self.state = GameState::GameOver;
        }
    }

    pub fn update_data(&mut self, view: &View) {
        let delta_time = mq::get_frame_time();

        self.spawn_timer += delta_time;
        if self.spawn_timer > SnakeGame::SPAWN_TIME {
            self.spawn_timer = 0.0;
            self.spawn_apples(view);
        }

        self.snake.update(delta_time);
    }

    fn spawn_apples(&mut self, view: &View) {
        if self.apples.len() < SnakeGame::MAX_APPLES {
            let mut pos= Point { x: 0, y: 0 }; 
            loop {
                let x = mq::rand::rand() % view.get_width(); 
                let y = mq::rand::rand() % view.get_height(); 
                pos = Point { x: x as i32, y: y as i32 };

                if pos.x < 1 || pos.x > view.get_width() as i32 - 2 {
                    continue
                }

                if pos.y < 1 || pos.y > view.get_width() as i32 - 2 {
                    continue
                }

                if self.snake.covered_position(&pos) {
                    continue;
                }

                break;
            }
            self.apples.push(pos);
        }
    }

    pub fn update_view(&mut self, view: &mut View) {
        let delta_time = mq::get_frame_time();
        view.update(delta_time);
    }

    pub fn draw(&self, view: &View) {
        let width: f32 = mq::screen_width() as f32;
        let height: f32 = mq::screen_height() as f32;

        mq::clear_background(mq::BLACK);
        view.draw();

        if self.state != GameState::GetReady {
            // Draw snake
            self.snake.draw(view);

            // Draw apples
            for apple in &self.apples {
                view.draw_block(apple, &mq::RED);
            }

            if self.state == GameState::GameOver {
                let text = "Gameover";
                let fontsize: u16 = 50;
                let dimension = mq::measure_text(text, None, fontsize, 1.0); 
                let x = width / 2.0 - dimension.width / 2.0;
                let y = height / 2.0 - dimension.height / 2.0 
                    + dimension.offset_y / 2.0;
                mq::draw_text("Gameover", x, y, fontsize as f32, mq::WHITE); 
            }
        }  
    }
}
