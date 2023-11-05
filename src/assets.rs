use macroquad::prelude::*;
use macroquad::audio::*;
use crate::datapakloader::DataPakLoader;


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
    pub const TEXTURE_COUNT: usize = 4;

    pub const SND_DEAD: usize = 0;
    pub const SND_EAT: usize = 1;
    pub const SND_GET_READY: usize = 2;
    pub const SND_MOVE: usize = 3;
    pub const SOUND_COUNT: usize = 4;

    pub const TTF_ELEGANT: usize = 0;
    pub const TTF_RETRO: usize = 1;
    pub const FONT_COUNT: usize = 2;

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

    pub async fn load_from_datapak(&mut self) {
        let mut reader = DataPakLoader::new("data.pak");

        for i in 0..Assets::TEXTURE_COUNT { 
            println!("Loading image #{}", i + 1);
            let texture = Texture2D::from_file_with_format(reader.load_image(i), 
                Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Nearest);
            self.textures.push(texture);
        }

        for i in 0..Assets::SOUND_COUNT { 
            println!("Loading sound #{}", i + 1);
            let result = load_sound_from_bytes(reader.load_sound(i)).await;
            match result {
                Ok(sound) => self.sounds.push(sound),
                Err(_e) => panic!("Error loading sound!")
            }
        }

        for i in 0..Assets::FONT_COUNT { 
            println!("Loading font #{}", i + 1);
            let font = match load_ttf_font_from_bytes(reader.load_font(i)) {
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
