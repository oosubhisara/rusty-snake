use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::audio::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TextureId {
    Wall, Snake
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SoundId {
    Move, Eat, Dead
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FontId {
    Main
}

pub struct Assets {
    textures: HashMap<TextureId, Texture2D>,
    sounds: HashMap<SoundId, Sound>,
    fonts: HashMap<FontId, Font>,
}
    

impl Assets {
    pub fn new() -> Assets {
        Assets { 
            textures: HashMap::new(), 
            sounds: HashMap::new(),
            fonts: HashMap::new() 
        }
    }

    pub async fn load(&mut self) {
        self.add_texture(TextureId::Wall, "assets/images/wall.png").await;
        self.add_texture(TextureId::Snake, "assets/images/snake.png").await;
        
        self.add_sound(SoundId::Move, "assets/sounds/move.wav").await;
        self.add_sound(SoundId::Eat, "assets/sounds/eat.wav").await;
        self.add_sound(SoundId::Dead, "assets/sounds/dead.wav").await;

        self.add_font(FontId::Main, "assets/fonts/FiraSans-Bold.ttf").await;
        println!("Asset loaded.");
    }

    async fn add_texture(&mut self, key: TextureId, filename: &str) {
        let result = load_texture(filename).await;
        match result {
            Ok(texture) => {
                texture.set_filter(FilterMode::Nearest);
                self.textures.insert(key, texture);
            },
            Err(_e) => {
                panic!("Error loading texture: {}", filename);
            }
        }
    }

    async fn add_sound(&mut self, key: SoundId, filename: &str) {
        let result = load_sound(filename).await;
        match result {
            Ok(sound) => {
               self.sounds.insert(key, sound);
            },
            Err(_e) => {
                panic!("Error loading sound: {}", filename);
            }
        }
    }

    async fn add_font(&mut self, key: FontId, filename: &str) {
        let result = load_ttf_font(filename).await;
        match result {
            Ok(font) => {
               self.fonts.insert(key, font);
            },
            Err(_e) => {
                panic!("Error loading font: {}", filename);
            }
        }
    }

    pub fn play_sound(&self, key: SoundId) {
        let sound: &Sound = self.sounds.get(&key).unwrap();
        play_sound_once(*sound);
    }

    pub fn get_texture(&self, key: TextureId) -> &Texture2D {
        let texture: &Texture2D = self.textures.get(&key).unwrap();
        &texture
    }

    pub fn get_font(&self, key: FontId) -> &Font {
        let font: &Font = self.fonts.get(&key).unwrap();
        &font
    }
}
