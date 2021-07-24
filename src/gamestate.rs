use macroquad::prelude::*;
use crate::assets::*;
use crate::gamescene::*;
use crate::snake::*;
use crate::apple::*;
use crate::common::*;

//=============================================================================
//    GameState
//=============================================================================
#[derive(Eq, PartialEq)]
pub enum LevelState {
    GetReady, Playing, Stunned, Dying, GameOver
}

pub struct GameState {
    assets: Assets,
    game_scene: GameScene,
    players: Vec<Snake>,
    apples: Vec<Apple>,
    spawn_timer: Timer,
    delay_timer: Timer,
    basic_gfx: bool,
    pub substate: LevelState,
}

impl GameState {
    pub fn new(width: f32, height: f32, grid_size: f32, player_count: u8) -> GameState {
        const SPAWN_TIME: f32 = 2.0;

        GameState { 
            assets: Assets::new(),
            game_scene: GameScene::new(width, height, grid_size), 
            players: { 
                let bound = Rect::new(1.0, 1.0, width - 2.0, height - 2.0);
                let mut players: Vec<Snake> = Vec::new();
                match player_count {
                    1 => players.push(Snake::new(GREEN, Vec2::new(10.0, 18.0), bound)),
                    _ => {
                        players.push(Snake::new(GREEN, Vec2::new(17.0, 18.0), bound));
                        players.push(Snake::new(PINK, Vec2::new(2.0, 18.0), bound));
                    }
                }
                players
            },
            apples: Vec::new(),
            spawn_timer: Timer::new(SPAWN_TIME), 
            delay_timer: Timer::new(Snake::STUN_INTERVAL), 
            basic_gfx: false, 
            substate: LevelState::GetReady
        }
    }

    pub async fn load(&mut self) {
        self.assets.load().await;
    }

    pub fn start(&self) {
        self.assets.play_sound(SoundId::GetReady);
    }

    pub fn reset(&mut self) {
        self.game_scene.reset();

        for player in &mut self.players {
            player.reset();
        }

        self.apples.clear();
        self.spawn_timer.reset(); 
        self.delay_timer.reset(); 
        self.substate = LevelState::GetReady;
        self.assets.play_sound(SoundId::GetReady);
    }

    pub fn handle_input(&mut self) {
        match self.substate {
            LevelState::Playing => {
                let mut dir_changed = 
                    if is_key_down(KeyCode::Up) {
                        self.players[0].set_direction(Direction::Up)
                    } else if is_key_down(KeyCode::Down) {
                        self.players[0].set_direction(Direction::Down)
                    } else if is_key_down(KeyCode::Left) {
                        self.players[0].set_direction(Direction::Left)
                    } else if is_key_down(KeyCode::Right) {
                        self.players[0].set_direction(Direction::Right)
                    } else {
                        false
                    };

                dir_changed = dir_changed || 
                    if is_key_down(KeyCode::W) {
                        self.players[1].set_direction(Direction::Up)
                    } else if is_key_down(KeyCode::S) {
                        self.players[1].set_direction(Direction::Down)
                    } else if is_key_down(KeyCode::A) {
                        self.players[1].set_direction(Direction::Left)
                    } else if is_key_down(KeyCode::D) {
                        self.players[1].set_direction(Direction::Right)
                    } else {
                        false
                    };

                if dir_changed {
                    self.assets.play_sound(SoundId::Move);
                }
            },
            LevelState::GameOver => {
                if is_key_down(KeyCode::Enter) || is_key_down(KeyCode::Space) {
                    self.reset();
                }
            },
            _ => { }
        }

        // Switch graphics style
        if is_key_pressed(KeyCode::F2) {
            self.basic_gfx = !self.basic_gfx;
        }

    }

    pub fn update(&mut self) {
        self.handle_input();

        match &self.substate {
            LevelState::GetReady => {
                if !self.game_scene.animate_grid() {
                    self.substate = LevelState::Playing;
                }
            },
            LevelState::Playing => {
                self.update_actors();
            },
            LevelState::Stunned => {
                if self.delay_timer.update() {
                    self.substate = LevelState::Dying;
                    self.delay_timer.reset();
                }
            }
            LevelState::Dying => {
                self.dying();
            }
            LevelState::GameOver => { }
        }
    }

    pub fn draw(&mut self) {
        let width: f32 = screen_width() as f32;
        let height: f32 = screen_height() as f32;

        if self.basic_gfx {
            clear_background(BLACK);
            self.game_scene.draw_basic();
            
            for player in &self.players {
                player.draw_basic(&self.game_scene);
            }

            for apple in &mut self.apples {
                apple.draw_basic(&self.game_scene);
            }
        } else {
            clear_background(Color::new(0.325, 0.133, 0.067, 1.0));
            self.game_scene.draw(self.assets.texture(TextureId::Wall));

            let mut player_index = 0;
            for player in &self.players {
                player.draw(&self.assets.texture(
                        if player_index == 0 { TextureId::Snake1 }
                        else { TextureId::Snake2 }
                        ), &self.game_scene);
                player_index += 1;
            }

            for apple in &mut self.apples {
                apple.draw(&self.assets.texture(TextureId::Apple), &self.game_scene);
            }
        }

        let mut text_params = TextParams {
            font: *self.assets.font(
                if self.basic_gfx { FontId::Retro } 
                else { FontId::Main } ),
            font_size: if self.basic_gfx { 24 } else { 30 },
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE
        };

        // Draw status text
        {
            let mut i = 0;

            for player in &self.players {
                let text = format!("Length: {}", player.length());
                text_params.color = player.color;
                let dimension = measure_text(&text, Some(text_params.font), 
                    text_params.font_size, text_params.font_scale);
                let offset = 32.0;
                let y = height - 30.0;
                match i {
                    0 => draw_text_ex(&text, width - offset - dimension.width, y, text_params),
                    _ => draw_text_ex(&text, offset, y, text_params)
                }
                i += 1;
            }
        }

        text_params.font_size = 48;
        text_params.color = WHITE;

        // Draw get ready text
        if self.substate == LevelState::GetReady {
            let text = "Get Ready";
            let pos = text_center_pos(text, text_params, width, height);
            draw_text_ex(text, pos.x, pos.y, text_params);
        } 

        // Draw game over text
        if self.substate == LevelState::GameOver {
            let text = "Gameover";
            let pos = text_center_pos(text, text_params, width, height);
            draw_text_ex(text, pos.x, pos.y, text_params);
        }
        
    }

    pub fn update_actors(&mut self) {
        const MAX_APPLES: usize = 3;

        if self.spawn_timer.update() && self.apples.len() < MAX_APPLES {
            let apple = Apple::random_spawn(&self.game_scene.play_area(), &self.players);
            self.apples.push(apple);
            self.spawn_timer.reset();
        }

        for apple in &mut self.apples {
            apple.update();
        }

        for player in &mut self.players {
            if player.check_update_time() {
                if player.check_collision() {
                    self.assets.play_sound(SoundId::Dead);
                    self.substate = LevelState::Stunned;
                } else {
                    player.update();
                    if player.eat_apples(&mut self.apples) {
                        self.assets.play_sound(SoundId::Eat);
                    }
                }
            }
        }
    }

    fn dying(&mut self) {
        for player in &mut self.players {
            if !player.is_alive() && !player.dying() {
                self.substate = LevelState::GameOver;
            }
        }
    }
}
