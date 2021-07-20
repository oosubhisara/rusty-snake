use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::audio::*;
use crate::gamescene::*;
use crate::actors::*;
use crate::common::*;

//=============================================================================
//    GameState
//=============================================================================
#[derive(Eq, PartialEq)]
pub enum GameSubState {
    GetReady, Playing, Stunned, Dying, GameOver
}

pub struct GameState {
    game_scene: GameScene,
    snake: Snake,
    apples: Vec<Apple>,
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
    font: Option<Font>,
    spawn_timer: Timer,
    delay_timer: Timer,
    pub substate: GameSubState,
}

impl GameState {
    const SNAKE_INITIAL_X: f32 = 3.0;
    const SNAKE_INITIAL_Y: f32 = 12.0;
    const SPAWN_TIME: f32 = 2.0;
    const MAX_APPLES: usize = 3;

    pub fn new(width: f32, height: f32, grid_size: f32) 
            -> GameState {
        let game_scene = GameScene::new(width, height, grid_size);
        let mut snake = Snake::new(
            GameState::SNAKE_INITIAL_X, GameState::SNAKE_INITIAL_Y);
        snake.set_bound(
            &Rect::new(1.0, 1.0, 
                       game_scene.get_width() as f32 - 2.0, 
                       game_scene.get_height() as f32 - 2.0)
        );
        let apples = Vec::new();
        let textures = HashMap::new();
        let sounds = HashMap::new();
        let font = None; 
        let spawn_timer = Timer::new(GameState::SPAWN_TIME); 
        let delay_timer = Timer::new(Snake::STUN_INTERVAL); 
        let substate = GameSubState::GetReady;

        GameState { game_scene, snake, apples, textures, sounds, font, 
                    spawn_timer, delay_timer, substate }
    }

    pub fn reset(&mut self) {
        self.game_scene.reset();
        self.snake.reset(
            GameState::SNAKE_INITIAL_X, GameState::SNAKE_INITIAL_Y);
        self.apples.clear();
        self.spawn_timer.reset(); 
        self.delay_timer.reset(); 
        self.substate = GameSubState::GetReady;
    }

    pub async fn load(&mut self) {
        self.load_texture("wall", "assets/images/wall.png").await;
        self.load_sound("move", "assets/sounds/move.wav").await;
        self.load_sound("eat", "assets/sounds/eat.wav").await;
        self.load_sound("dead", "assets/sounds/dead.wav").await;
        self.font = Some(
            load_ttf_font("assets/fonts/FiraSans-Bold.ttf").await.unwrap());
    }

    async fn load_texture(&mut self, key: &str, filename: &str) {
        let result = load_texture(filename).await;
        match result {
            Ok(x) => {
                x.set_filter(FilterMode::Nearest);
               self.textures.insert(String::from(key), x);
            },
            Err(_e) => {
                panic!("Error loading texture: {}", filename);
            }
        }
    }

    async fn load_sound(&mut self, key: &str, filename: &str) {
        let result = load_sound(filename).await;
        match result {
            Ok(x) => {
               self.sounds.insert(String::from(key), x);
            },
            Err(_e) => {
                panic!("Error loading sound: {}", filename);
            }
        }
    }

    pub fn handle_input(&mut self) {
        match self.substate {
            GameSubState::Playing => {
                let mut dir_changed = false;
                if is_key_down(KeyCode::Up) {
                    dir_changed = self.snake.set_direction(Direction::UP);
                }
                else if is_key_down(KeyCode::Down) {
                    dir_changed = self.snake.set_direction(Direction::DOWN);
                }
                else if is_key_down(KeyCode::Left) {
                    dir_changed = self.snake.set_direction(Direction::LEFT);
                }
                else if is_key_down(KeyCode::Right) {
                    dir_changed = self.snake.set_direction(Direction::RIGHT);
                }

                if dir_changed {
                    self.play_sound("move");
                }
            },
            GameSubState::GameOver => {
                if is_key_down(KeyCode::Enter) {
                    self.reset();
                }
            },
            _ => { }
        }
    }

    pub fn update(&mut self) {
        match &self.substate {
            GameSubState::GetReady => {
                if !self.game_scene.animate_grid() {
                    self.substate = GameSubState::Playing;
                }
            },
            GameSubState::Playing => {
                self.handle_input();
                self.update_actors();
            },
            GameSubState::Stunned => {
                if self.delay_timer.update() {
                    self.substate = GameSubState::Dying;
                    self.delay_timer.reset();
                }
            }
            GameSubState::Dying => {
                self.dying();
            }
            GameSubState::GameOver => { 
                self.handle_input();
            }
        }
    }

    pub fn draw(&mut self) {
        let width: f32 = screen_width() as f32;
        let height: f32 = screen_height() as f32;
        let text_params = TextParams {
            font: self.font.unwrap(),
            font_size: 48,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE
        };

        clear_background(Color::new(0.325, 0.133, 0.067, 1.0));
        // Draw scene
        self.game_scene.draw(self.get_texture("wall"));

        // Draw snake
        self.snake.draw(&self.game_scene);

        // Draw apples
        for apple in &self.apples {
            let color = Color::new(1.0, 0.0, 0.0, apple.alpha);
            self.game_scene.draw_circle(&apple.pos, &color);
        }

        // Draw get ready text
        if self.substate == GameSubState::GetReady {
            let text = "Get Ready";
            let pos = get_text_center_pos(text, text_params, width, height);
            draw_text_ex(text, pos.x, pos.y, text_params);
        } 

        // Draw game over text
        if self.substate == GameSubState::GameOver {
            let text = "Gameover";
            let pos = get_text_center_pos(text, text_params, width, height);
            draw_text_ex(text, pos.x, pos.y, text_params);
        }
        
    }

    pub fn update_actors(&mut self) {
        if self.spawn_timer.update() {
            self.spawn_apples();
            self.spawn_timer.reset();
        }

        for apple in &mut self.apples {
            apple.update();
        }

        if self.snake.update() {
            if !self.snake.is_alive() {
                self.play_sound("dead");
                //play_sound_once(self.sounds[2]);
                self.substate = GameSubState::Stunned;
            } else {
                if self.snake.eat_apples(&mut self.apples) {
                    self.play_sound("eat");
                    //play_sound_once(self.sounds[1]);
                }
            }
        }
    }

    fn dying(&mut self) {
        if !self.snake.dying() {
            self.substate = GameSubState::GameOver;
        }
    }

    fn spawn_apples(&mut self) {
        if self.apples.len() < GameState::MAX_APPLES {
            let mut pos: Vec2;
            let width = self.game_scene.get_width() - 2.0;
            let height = self.game_scene.get_height() - 2.0;

            loop {
                // Randomize position inside playing field excluding borders
                let x: f32 = (rand::rand() % width as u32) as f32;
                let y: f32 = (rand::rand() % height as u32) as f32;
                // +1 to start after left & top border
                pos = Vec2::new(x + 1.0, y + 1.0); 

                // Do not spawn on top of the snake
                if self.snake.is_position_overlapped(&pos) {
                    continue;
                }

                break;
            }
            
            //DEBUG println!("Spawned an apple at {},{}", pos.x, pos.y);
            let apple = Apple::new(&pos);
            self.apples.push(apple);  // add to apple list
        }
    }

    fn play_sound(&self, key: &str) {
        let sound: &Sound = self.sounds.get(&String::from(key)).unwrap();
        play_sound_once(*sound);
    }

    fn get_texture(&self, key: &str) -> &Texture2D {
        let texture: &Texture2D = self.textures.get(
            &String::from(key)).unwrap();
        texture
    }
}
