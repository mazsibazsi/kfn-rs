
/// This struct represents a text buffer and it's parameters.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    pub text_events: Vec<crate::helpers::event::Event>,
    pub font: speedy2d::font::Font,
    pub font_size: f32,
    pub inactive_color: speedy2d::color::Color,
    pub inactive_outline_color: speedy2d::color::Color,
    pub active_color: speedy2d::color::Color,
    pub active_outline_color: speedy2d::color::Color,
    pub outline_weight: i32,
}

/// This module is supposed to separate the functions, that draw the text buffer.
pub mod text_buffer {

    use speedy2d::{Graphics2D, font::{TextLayout, TextOptions}};

    use crate::{kfn_player::KfnPlayer, helpers::event::EventType};
    

    impl KfnPlayer {
        pub fn draw_text_buffer(&mut self, graphics: &mut Graphics2D) {

            let current_time = (self.time.offset + self.time.start_time.elapsed()).as_millis(); 
        
            if (self.text_buffer.text_events[self.text_buffer.text_events.len()-2].time * 10) as u128 <= current_time  {
                self.text_buffer.text_events.pop();
            }

            let text_inactive = match &self.text_buffer.text_events[self.text_buffer.text_events.len()-1].event_type {
                EventType::Text(s) => Into::<String>::into(s.to_owned()),
                _ => "".to_string()
            };

            let text_active = match &self.text_buffer.text_events[self.text_buffer.text_events.len()-1].event_type {
                EventType::Text(s) => {
                    let mut temp: String = String::new();
                    for (time, fragment) in s.fragments.clone() {
                        if (time*10) as u128 <= current_time {
                            temp.push_str(fragment.as_str());
                        }
                    }
                    Into::<String>::into(temp.to_owned())
                },
                _ => " ".to_string()
            };
            


            let ftext_full = self.text_buffer.font.layout_text(
                &text_inactive,
                self.text_buffer.font_size,
                TextOptions::new()
            );

            let ftext_elapsed = self.text_buffer.font.layout_text(
                &text_active,
                self.text_buffer.font_size,
                TextOptions::new()
            );


    
            let outline_ftext_full = self.text_buffer.font.layout_text(
                &text_inactive,
                self.text_buffer.font_size,
                TextOptions::new()
            );

            let outline_ftext_elapsed = self.text_buffer.font.layout_text(
                &text_active,
                self.text_buffer.font_size,
                TextOptions::new()
            );

            
            let center_x: f32 = ((self.window_size.x) as f32 / 2.0) - (ftext_full.width() - ftext_full.width() / 2.0);
            let center_y: f32 = ((self.window_size.y) as f32 / 2.0) - (ftext_full.height() - ftext_full.height() / 2.0);
    
            // drawing the outline here
            for n in 0..self.text_buffer.outline_weight {
                let outline_color = speedy2d::color::Color::from_rgba(
                    self.text_buffer.inactive_outline_color.r(), 
                    self.text_buffer.inactive_outline_color.g(), 
                    self.text_buffer.inactive_outline_color.b(), 
                    self.text_buffer.inactive_outline_color.a()-(2.0/n as f32));

                // this is achieved by drawing first the outlines
                graphics.draw_text((center_x, center_y+n as f32), outline_color, &outline_ftext_full);
                graphics.draw_text((center_x+n as f32, center_y-n as f32), outline_color, &outline_ftext_full);
                graphics.draw_text((center_x, center_y-n as f32), outline_color, &outline_ftext_full);
                graphics.draw_text((center_x+n as f32, center_y+n as f32), outline_color, &outline_ftext_full);
                graphics.draw_text((center_x+n as f32, center_y), outline_color, &outline_ftext_full);
                graphics.draw_text((center_x-n as f32, center_y+n as f32), outline_color, &outline_ftext_full);
                graphics.draw_text((center_x-n as f32, center_y), outline_color, &outline_ftext_full);
                graphics.draw_text((center_x-n as f32, center_y-n as f32), outline_color, &outline_ftext_full);
            }

            // and then drawing the actual text
            graphics.draw_text((center_x, center_y), self.text_buffer.inactive_color, &ftext_full);
            graphics.draw_text((center_x, center_y), self.text_buffer.active_color, &ftext_elapsed);

            /* // NEXT TEXT

            let text_next = match &self.text_buffer.text_events[self.text_buffer.text_events.len()-2].event_type {
                EventType::Text(s) => Into::<String>::into(s.to_owned()),
                _ => "".to_string()
            };

            let ftext_next = self.text_buffer.font.layout_text(
                &text_next,
                self.text_buffer.font_size,
                TextOptions::new()
            );


            let outline_ftext_next = self.text_buffer.font.layout_text(
                &text_next,
                self.text_buffer.font_size,
                TextOptions::new()
            );    

            let center_x: f32 = ((self.window_size.x) as f32 / 2.0) - (ftext_next.width() - ftext_next.width() / 2.0);
            let center_y: f32 = ((self.window_size.y) as f32 / 2.0) - (ftext_next.height() - ftext_next.height() / 2.0)+200.0;
    
            // drawing the outline here
            for n in 0..self.text_buffer.outline_weight {
                let outline_color = speedy2d::color::Color::from_rgba(
                    self.text_buffer.outline_color.r(), 
                    self.text_buffer.outline_color.g(), 
                    self.text_buffer.outline_color.b(), 
                    self.text_buffer.outline_color.a()-(2.0/n as f32));

                // this is achieved by drawing first the outlines
                graphics.draw_text((center_x, center_y+n as f32), outline_color, &outline_ftext_next);
                graphics.draw_text((center_x+n as f32, center_y-n as f32), outline_color, &outline_ftext_next);
                graphics.draw_text((center_x, center_y-n as f32), outline_color, &outline_ftext_next);
                graphics.draw_text((center_x+n as f32, center_y+n as f32), outline_color, &outline_ftext_next);
                graphics.draw_text((center_x+n as f32, center_y), outline_color, &outline_ftext_next);
                graphics.draw_text((center_x-n as f32, center_y+n as f32), outline_color, &outline_ftext_next);
                graphics.draw_text((center_x-n as f32, center_y), outline_color, &outline_ftext_next);
                graphics.draw_text((center_x-n as f32, center_y-n as f32), outline_color, &outline_ftext_next);
            }

            // and then drawing the actual text
            graphics.draw_text((center_x, center_y), self.text_buffer.color, &ftext_next);
 */

        }
    }
    
}