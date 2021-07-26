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
    GetReady, SnakeEntering, Playing, Stunned, Dying, GameOver
}

pub struct GameState {
    assets: Assets,
    game_scene: GameScene,
    players: Vec<Snake>,
    apples: Vec<Apple>,
    scores: [i32; 2],
    game_time: f32,
    spawn_timer: Timer,
    delay_timer: Timer,
    basic_actor: bool,
    basic_scene: bool,
    pub substate: LevelState,
}

impl GameState {
    pub fn new(width: f32, height: f32, grid_size: f32, player_count: u8) -> GameState {
        const SPAWN_TIME: f32 = 2.0;
        let game_scene = GameScene::new(width, height, grid_size);
        let left_gate = game_scene.left_gate_position();
        let right_gate = game_scene.right_gate_position();

        GameState { 
            assets: Assets::new(),
            game_scene: game_scene, 
            players: { 
                let bound = Rect::new(1.0, 1.0, width - 2.0, height - 2.0);
                let mut players: Vec<Snake> = Vec::new();
                match player_count {
                    1 => {
                        players.push(Snake::new(0, GREEN, Vec2::new(
                            right_gate.x, right_gate.y + 1.0), bound));
                    }
                    _ => {
                        players.push(Snake::new(0, GREEN, Vec2::new(
                            right_gate.x, right_gate.y + 1.0), bound));
                        players.push(Snake::new(1, PINK, Vec2::new(
                            left_gate.x, left_gate.y + 1.0), bound));
                    }
                }
                players
            },
            apples: Vec::new(),
            scores: [0, 0],
            game_time: 0.0,
            spawn_timer: Timer::new(SPAWN_TIME), 
            delay_timer: Timer::new(Snake::STUN_INTERVAL), 
            basic_actor: false, 
            basic_scene: true, 
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
        self.scores[0] = 0;
        self.scores[1] = 0;
        self.game_time = 0.0;
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
        if is_key_pressed(KeyCode::F3) {
            self.basic_actor = !self.basic_actor;
        } else if is_key_pressed(KeyCode::F4) {
            self.basic_scene = !self.basic_scene;
        }

    }

    pub fn update(&mut self) {
        self.handle_input();

        match &self.substate {
            LevelState::GetReady => {
                if !self.game_scene.animate_grid() {
                    self.substate = LevelState::SnakeEntering;
                }
            },
            LevelState::SnakeEntering => {
                self.update_actors();
                let player = &self.players[0];
                if player.position().y == self.game_scene.height() - 1.0 - player.length() as f32 {
                    self.substate = LevelState::Playing;
                    self.game_scene.close_gates();
                }
            },
            LevelState::Playing => {
                self.game_time += get_frame_time();
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
        if self.basic_scene {
            clear_background(BLACK);
            self.game_scene.draw_basic();
        } else {
            clear_background(Color::new(0.325, 0.133, 0.067, 1.0));
            self.game_scene.draw(self.assets.texture(TextureId::Wall));
        }
        self.draw_actors();
        self.draw_texts();
    }

    fn draw_actors(&mut self) {
        if self.basic_actor {
            let draw_player_order = if !self.players[0].is_alive() { [1, 0] } else { [0, 1] };
            for i in draw_player_order {
                self.players[i].draw_basic(&self.game_scene);
            }

            for apple in &mut self.apples {
                apple.draw_basic(&self.game_scene);
            }
        } else {
            let draw_player_order = if !self.players[0].is_alive() { [1, 0] } else { [0, 1] };
            
            for i in draw_player_order {
                self.players[i].draw(
                    &self.assets.texture( 
                        if self.players[i].id() == 0 { TextureId::Snake1 } 
                        else { TextureId::Snake2 }
                    ),
                    &self.game_scene);
            }

            for apple in &mut self.apples {
                apple.draw(&self.assets.texture(TextureId::Apple), &self.game_scene);
            }
        }
    }

    fn draw_texts(&mut self) {
        let width: f32 = screen_width() as f32;
        let height: f32 = screen_height() as f32;

        let mut text_params = TextParams {
            font: *self.assets.font(
                if self.basic_scene { FontId::Retro } 
                else { FontId::Main } ),
            font_size: if self.basic_scene { 24 } else { 30 },
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE
        };

        // Draw status text
        {
            let mut i = 0;

            for player in &self.players {
                let text = "Score: 0000000";
                text_params.color = player.color;
                let dimension = measure_text(&text, Some(text_params.font), 
                    text_params.font_size, text_params.font_scale);
                let x1 = 32.0;
                let x2 = width - 32.0 - dimension.width;
                let y1 = height - 55.0; 
                let y2 = height - 25.0; 
                match i {
                    0 => {
                        draw_text_ex(&format!("Length: {}", player.length())[..],
                            x2, y1, text_params);
                        draw_text_ex(&format!("Score: {:07}", self.scores[i])[..],
                            x2, y2, text_params);
                    },
                    _ => {
                        draw_text_ex(&format!("Length: {}", player.length())[..],
                            x1, y1, text_params);
                        draw_text_ex(&format!("Score: {:07}", self.scores[i])[..],
                            x1, y2, text_params);
                    }
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

    pub fn player_by_id(&self, id: i32) -> Option<&Snake> {
        if id >= self.players.len() as i32 {
            None
        } else {
            Some(&self.players[id as usize]) }
    }

    fn update_scores(&mut self) {
        for i in 0..self.players.len() {
            self.scores[i] +=  (self.players[i].length() as f32 * 0.2) as i32;
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

        let player_count = self.players.len();
        for i  in 0..player_count {
            if self.players[i].check_update_time() {
                let opponent_id: i32 = self.opponent_player_index(i as i32);
                if self.substate == LevelState::Playing 
                        && self.players[i].check_collision(&self, opponent_id) {
                    self.players[i].update();
                    self.players[i].kill_self();
                    self.assets.play_sound(SoundId::Dead);
                    self.substate = LevelState::Stunned;
                } else {
                    self.players[i].update();
                    if self.players[i].eat_apples(&mut self.apples) {
                        self.assets.play_sound(SoundId::Eat);
                    }
                }
                self.update_scores();
            }
        }
    }

    fn opponent_player_index(&self, id: i32) -> i32 { 
        if self.players.len() == 1 {
            -1
        } else if id == 0 { 
            1
        } else { 
            0
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
