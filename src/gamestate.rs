use macroquad::prelude::*;
use crate::assets::*;
use crate::gamescene::*;
use crate::snake::*;
use crate::apple::*;
use crate::common::*;
use crate::*;

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
    player_count: usize,
    players: [Snake; 2],
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
    pub fn new(width: f32, height: f32, grid_size: f32) -> GameState {
        const SPAWN_TIME: f32 = 2.0;
        let game_scene = GameScene::new(width, height, grid_size);
        let left_gate = game_scene.left_gate_position();
        let right_gate = game_scene.right_gate_position();

        GameState { 
            assets: Assets::new(),
            game_scene: game_scene, 
            player_count: 1,
            players: { 
                let bound = Rect::new(1.0, 1.0, width - 2.0, height - 2.0);
                [
                    Snake::new(0, GREEN, Vec2::new(right_gate.x, right_gate.y + 1.0), bound),
                    Snake::new(1, PINK, Vec2::new( left_gate.x, left_gate.y + 1.0), bound)
                ]
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
        self.assets.load_from_bundle().await;
    }

    pub fn start(&self) {
        self.assets.play_sound(Assets::SND_GET_READY);
    }

    pub fn reset(&mut self) {
        self.game_scene.reset();

        for i in 0..self.player_count {
            self.players[i].reset();
        }

        self.apples.clear();
        self.scores[0] = 0;
        self.scores[1] = 0;
        self.game_time = 0.0;
        self.spawn_timer.reset(); 
        self.delay_timer.reset(); 
        self.substate = LevelState::GetReady;
        self.assets.play_sound(Assets::SND_GET_READY);
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
                    self.assets.play_sound(Assets::SND_MOVE);
                }
            },
            LevelState::GameOver => {
                if is_key_down(KeyCode::Enter) || is_key_down(KeyCode::Space) {
                    self.reset();
                }
            },
            _ => { }
        }

        if is_key_pressed(KeyCode::F1) {
            self.player_count = 1;
            self.reset();
        } else if is_key_pressed(KeyCode::F2) {
            self.player_count = 2;
            self.reset();
        } else if is_key_pressed(KeyCode::F5) {
            self.basic_actor = !self.basic_actor;
        } else if is_key_pressed(KeyCode::F6) {
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
            self.game_scene.draw(self.assets.texture(Assets::TEX_WALL));
        }
        self.draw_actors();
        self.draw_texts();
    }

    fn draw_actors(&mut self) {
        if self.basic_actor {
            let draw_player_order = if !self.players[0].is_alive() { [1, 0] } else { [0, 1] };
            for i in draw_player_order {
                if i < self.player_count {
                    self.players[i].draw_basic(&self.game_scene);
                }
            }

            for apple in &mut self.apples {
                apple.draw_basic(&self.game_scene);
            }
        } else {
            let draw_player_order = if !self.players[0].is_alive() { [1, 0] } else { [0, 1] };
            
            for i in draw_player_order {
                if i < self.player_count {
                    self.players[i].draw(
                        &self.assets.texture( 
                            if self.players[i].id() == 0 { Assets::TEX_SNAKE1 } 
                            else { Assets::TEX_SNAKE2 }
                        ),
                        if self.substate == LevelState::GetReady { false } else { true },
                        &self.game_scene);
                }
            }

            for apple in &mut self.apples {
                apple.draw(&self.assets.texture(Assets::TEX_APPLE), &self.game_scene);
            }
        }
    }

    fn draw_texts(&mut self) {
        let width: f32 = WINDOW_WIDTH; 
        let height: f32 = WINDOW_HEIGHT; 

        let mut text_params = TextParams {
            font: *self.assets.font(
                if self.basic_scene { Assets::TTF_RETRO } 
                else { Assets::TTF_ELEGANT } ),
            font_size: if self.basic_scene { 24 } else { 30 },
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE
        };

        // Draw status text
        {
            for i in 0..self.player_count {
                let text = "Score: 0000000";
                text_params.color = self.players[i].color;
                let dimension = measure_text(&text, Some(text_params.font), 
                    text_params.font_size, text_params.font_scale);
                let x1 = 32.0;
                let x2 = width - 32.0 - dimension.width;
                let y1 = height - 55.0; 
                let y2 = height - 25.0; 
                match i {
                    0 => {
                        draw_text_ex(&format!("Length: {}", self.players[i].length())[..],
                            x2, y1, text_params);
                        draw_text_ex(&format!("Score: {:07}", self.scores[i])[..],
                            x2, y2, text_params);
                    },
                    _ => {
                        draw_text_ex(&format!("Length: {}", self.players[i].length())[..],
                            x1, y1, text_params);
                        draw_text_ex(&format!("Score: {:07}", self.scores[i])[..],
                            x1, y2, text_params);
                    }
                }
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
        if id < 0 || id >= self.player_count as i32 {
            None
        } else {
            Some(&self.players[id as usize]) }
    }

    fn update_scores(&mut self) {
        for i in 0..self.player_count {
            self.scores[i] +=  (self.players[i].length() as f32 * 0.2) as i32;
        }
    }

    pub fn update_actors(&mut self) {
        const MAX_APPLES: usize = 3;

        if self.spawn_timer.update() && self.apples.len() < MAX_APPLES {
            let apple = Apple::random_spawn(&self.game_scene.play_area(), 
                                            &self.players[0..self.player_count]);
            self.apples.push(apple);
            self.spawn_timer.reset();
        }

        for apple in &mut self.apples {
            apple.update();
        }

        for i  in 0..self.player_count {
            if self.players[i].check_update_time() {
                let opponent_id: i32 = self.opponent_player_index(i as i32);
                if self.substate == LevelState::Playing 
                        && self.players[i].check_collision(&self, opponent_id) {
                    self.players[i].update();
                    self.players[i].kill_self();
                    self.assets.play_sound(Assets::SND_DEAD);
                    self.substate = LevelState::Stunned;
                } else {
                    self.players[i].update();
                    if self.players[i].eat_apples(&mut self.apples) {
                        self.assets.play_sound(Assets::SND_EAT);
                    }
                }
                self.update_scores();
            }
        }
    }

    fn opponent_player_index(&self, id: i32) -> i32 { 
        if self.player_count == 1 {
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
