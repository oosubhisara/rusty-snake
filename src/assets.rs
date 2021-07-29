use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use macroquad::prelude::*;
use macroquad::audio::*;


pub struct Assets {
    textures: Vec<Texture2D>,
    sounds: Vec<Sound>,
    fonts: Vec<Font>
}
    
impl Assets {
    pub const TEX_APPLE: usize = 0;
    pub const TEX_SNAKE1: usize = 1;
    pub const TEX_SNAKE2: usize = 2;
    pub const TEX_WALL: usize = 3;

    pub const SND_MOVE: usize = 0;
    pub const SND_GET_READY: usize = 1;
    pub const SND_DEAD: usize = 2;
    pub const SND_EAT: usize = 3;

    pub const TTF_ELEGANT: usize = 0;
    pub const TTF_RETRO: usize = 1;

    pub fn new() -> Assets {
        Assets { 
            textures: Vec::new(), 
            sounds: Vec::new(),
            fonts: Vec::new() 
        }
    }

    pub async fn load(&mut self) {
        self.add_texture("assets/images/apple.png").await;
        self.add_texture("assets/images/snake1.png").await;
        self.add_texture("assets/images/snake2.png").await;
        self.add_texture("assets/images/wall.png").await;
        
        self.add_sound("assets/sounds/move.wav").await;
        self.add_sound("assets/sounds/get_ready.wav").await;
        self.add_sound("assets/sounds/dead.wav").await;
        self.add_sound("assets/sounds/eat.wav").await;

        self.add_font("assets/fonts/dpcomic.ttf").await;
        self.add_font("assets/fonts/gomarice_no_continue.ttf").await;
        println!("Asset loaded.");
    }

    pub async fn load_from_bundle(&mut self) {
        const image_toc: [[u32;2];4] = [
            [96, 414],
            [510, 3337],
            [3847, 3665],
            [7512, 392]
        ];
            
        const sound_toc: [[u32;2];4] = [
            [7904, 200260],
            [208164, 158398],
            [366562, 398468],
            [765030, 29878]
        ];

        const font_toc: [[u32;2];2] = [
            [794908, 8940],
            [803848, 20120]
        ];

        let mut bundle_file: File = File::open("game.dat").unwrap();
        let mut buf: Vec<u8> = Vec::new(); 
        bundle_file.read_to_end(&mut buf);

        for info in image_toc { 
            let slice_from: usize = info[0] as usize;
            let slice_to: usize = slice_from + info[1] as usize;
            let texture = Texture2D::from_file_with_format(&buf[slice_from..slice_to], 
                Some(ImageFormat::Png));
            self.textures.push(texture);
        }

        for info in sound_toc { 
            let slice_from: usize = info[0] as usize;
            let slice_to: usize = slice_from + info[1] as usize;
            let result = load_sound_from_bytes(&buf[slice_from..slice_to]).await;
            match result {
                Ok(sound) => self.sounds.push(sound),
                Err(_e) => panic!("Error loading sound!")
            }
        }

        for info in font_toc { 
            let slice_from: usize = info[0] as usize;
            let slice_to: usize = slice_from + info[1] as usize;
            let font = match load_ttf_font_from_bytes(&buf[slice_from..slice_to]) {
                Ok(font) => font,
                Err(_e) => panic!("Error loading font!")
            };

            self.fonts.push(font);
        }

        println!("Asset loaded.");
    }

    async fn add_texture(&mut self, filename: &str) {
        let result = load_texture(filename).await;
        match result {
            Ok(texture) => {
                texture.set_filter(FilterMode::Nearest);
                self.textures.push(texture);
            },
            Err(_e) => {
                panic!("Error loading texture: {}", filename);
            }
        }
    }

    async fn add_sound(&mut self, filename: &str) {
        let result = load_sound(filename).await;
        match result {
            Ok(sound) => {
               self.sounds.push(sound);
            },
            Err(_e) => {
                panic!("Error loading sound: {}", filename);
            }
        }
    }

    async fn add_font(&mut self, filename: &str) {
        let result = load_ttf_font(filename).await;
        match result {
            Ok(font) => {
               self.fonts.push(font);
            },
            Err(_e) => {
                panic!("Error loading font: {}", filename);
            }
        }
    }

    pub fn play_sound(&self, id: usize) {
        let sound: &Sound = self.sounds.get(id).unwrap();
        play_sound_once(*sound);
    }

    pub fn texture(&self, id: usize) -> &Texture2D {
        let texture: &Texture2D = self.textures.get(id).unwrap();
        &texture
    }

    pub fn font(&self, id: usize) -> &Font {
        let font: &Font = self.fonts.get(id).unwrap();
        &font
    }
}
