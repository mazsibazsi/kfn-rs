
/// This struct represents a text buffer and it's parameters.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    pub text_events: Vec<crate::helpers::event::Event>,
    pub font: speedy2d::font::Font,
    pub font_size: f32,
    pub color: speedy2d::color::Color,
    pub outline_color: speedy2d::color::Color,
    pub outline_weight: i32,
}

/// This module is supposed to separate the functions, that draw the text buffer.
pub mod text_buffer {

    use speedy2d::{Graphics2D, font::{TextLayout, TextOptions}};

    use crate::{kfn_player::KfnPlayer, helpers::event::EventType};
    

    impl KfnPlayer {
        pub fn draw_text_buffer(&mut self, graphics: &mut Graphics2D) {
        
            if (self.text_buffer.text_events[self.text_buffer.text_events.len()-2].time * 10) as u128 <= (self.time.offset + self.time.start_time.elapsed()).as_millis()  {
                self.text_buffer.text_events.pop();
            }
            let text = match &self.text_buffer.text_events[self.text_buffer.text_events.len()-1].event_type {
                EventType::Text(s) => Into::<String>::into(s.to_owned()),
                _ => "".to_string()
            };
            
            let ftext = self.text_buffer.font.layout_text(
                &text,
                self.text_buffer.font_size,
                TextOptions::new()
            );
    
            let outline_ftext = self.text_buffer.font.layout_text(
                &text,
                self.text_buffer.font_size,
                TextOptions::new()
            );
    
            
            let center_x: f32 = ((self.window_size.x) as f32 / 2.0) - (ftext.width() - ftext.width() / 2.0);
            
    
            // drawing the outline here
            for n in 0..self.text_buffer.outline_weight {
                let outline_color = speedy2d::color::Color::from_rgba(
                    self.text_buffer.outline_color.r(), 
                    self.text_buffer.outline_color.g(), 
                    self.text_buffer.outline_color.b(), 
                    self.text_buffer.outline_color.a()-(2.0/n as f32));

                // this is achieved by drawing first the outlines
                graphics.draw_text((center_x, 200.0+n as f32), outline_color, &outline_ftext);
                graphics.draw_text((center_x+n as f32, 200.0-n as f32), outline_color, &outline_ftext);
                graphics.draw_text((center_x, 200.0-n as f32), outline_color, &outline_ftext);
                graphics.draw_text((center_x+n as f32, 200.0+n as f32), outline_color, &outline_ftext);
                graphics.draw_text((center_x+n as f32, 200.0), outline_color, &outline_ftext);
                graphics.draw_text((center_x-n as f32, 200.0+n as f32), outline_color, &outline_ftext);
                graphics.draw_text((center_x-n as f32, 200.0), outline_color, &outline_ftext);
                graphics.draw_text((center_x-n as f32, 200.0-n as f32), outline_color, &outline_ftext);
            }
    
            // and then drawing the actual text
            graphics.draw_text((center_x, 200.0), self.text_buffer.color, &ftext);
        }
    }
    
}