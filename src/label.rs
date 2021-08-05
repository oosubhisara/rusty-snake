use macroquad::prelude::*;

//=============================================================================
//    Label
//=============================================================================
pub struct Label {
    text: String,
    pos: Vec2,
    text_params: TextParams,
    shadow_color: Color,
    shadow_offset: Vec2,
    dimensions: TextDimensions
}

impl Label {
    pub fn new() -> Label {
        Label {
            text: String::from(""), 
            pos: Vec2::ZERO,
            text_params: TextParams::default(),
            shadow_color: BLACK,
            shadow_offset: Vec2::ZERO,
            dimensions: TextDimensions { 
                width: 0.0,
                height: 0.0,
                offset_y: 0.0
            }
        }
    }

    pub fn set_shadow(&mut self, offset: &Vec2, color: &Color) -> &mut Label {
        self.shadow_offset = offset.clone();
        self.shadow_color = color.clone();
        self
    }

    pub fn set_text(&mut self, text: &str) -> &mut Label {
        self.text = text.to_string();
        self.update();
        self
    }

    pub fn set_position(&mut self, pos: &Vec2) -> &mut Label {
        self.pos = pos.clone();  
        self
    }

    pub fn set_font(&mut self, font: &Font, font_size: u16) -> &mut Label {
        self.text_params.font = font.clone();
        self.text_params.font_size = font_size;
        self.update();
        self
    }

    pub fn set_color(&mut self, color: &Color) -> &mut Label {
        self.text_params.color = color.clone(); self
    }

    pub fn center(&mut self, x: Option<f32>, y: Option<f32>, rect: &Rect) -> &mut Label {
        self.pos.x = match x {
            Some(x) => x,
            None => rect.x + rect.w / 2.0 - self.dimensions.width / 2.0
        };

        self.pos.y = match y {
            Some(y) => y, 
            None => rect.y + rect.h / 2.0 - self.dimensions.height / 2.0 
                    - self.dimensions.offset_y / 2.0
        };

        self
    }

    pub fn width(&self) -> f32 {
        self.dimensions.width
    }

    pub fn height(&self) -> f32 {
        self.dimensions.height
    }

    pub fn right(&self) -> f32 {
        self.pos.x + self.dimensions.width
    }

    pub fn bottom(&self) -> f32 {
        self.pos.y + self.dimensions.height
    }

    pub fn draw(&mut self) {
        if self.shadow_offset != Vec2::ZERO {
            let mut shadow_params = self.text_params.clone();
            shadow_params.color = self.shadow_color; 
            draw_text_ex(
                &self.text.as_str(), 
                self.pos.x + self.shadow_offset.x, 
                self.pos.y + self.shadow_offset.y + self.dimensions.offset_y, 
                shadow_params
            );
        }

        draw_text_ex(&self.text.as_str(), self.pos.x, self.pos.y + self.dimensions.offset_y,
                     self.text_params);
    }

    fn update(&mut self) {
        self.dimensions = measure_text(
                &self.text.as_str(), Some(self.text_params.font), self.text_params.font_size, 1.0
        ); 
    }
}
