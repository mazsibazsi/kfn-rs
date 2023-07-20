pub mod window_handler {
    use speedy2d::{window::{WindowHandler, WindowHelper, WindowStartupInfo}, dimen::Vector2, Graphics2D, font::{TextLayout, TextOptions}};

    use crate::{kfn_player::KfnPlayer, helpers::event::EventType, kfn_ini::eff::Action};

    impl WindowHandler for KfnPlayer {
        fn on_resize(
                &mut self,
                _helper: &mut WindowHelper<()>,
                size_pixels: Vector2<u32>
            ) {
                self.window_size.x = size_pixels.x;
                self.window_size.y = size_pixels.y;
                self.screen_buffer.resized = true;
    
        }
    
        fn on_start(&mut self, helper: &mut WindowHelper<()>, _info: WindowStartupInfo)  {
            helper.set_resizable(true);
            helper.set_icon_from_rgba_pixels(
                image::open("src/icons/icon32x32.png").unwrap().into_bytes(), (32, 32)).unwrap();
            self.set_initial_state();
        }
    
        fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
            
            
            // routine for displaying framerate
            let draw_start = std::time::Instant::now();
            let text = 
                &self.diag.1.font.layout_text(
                    &std::format!(
                        "Frame: {}, FPS: {:.0}, frame draw time: {:.0} ms",
                        self.diag.1.counter,
                        self.diag.1.fps,
                        self.diag.1.draw_time/1000.0),
                        42.0,
                        TextOptions::new());
            
            // draw routine
            // only executes, when not paused
            if !self.paused {
                
                // clear screen
                graphics.clear_screen(speedy2d::color::Color::BLACK);
    
    
    
                // look for incoming events
                while !self.receiver.is_empty() {
                    match self.receiver.try_recv() {
                        Ok(event_recv) => {
                            println!("{} received", event_recv.time);
                            dbg!(&event_recv);
                            self.event_queue.push(event_recv);
                                
        
                        },
                        Err(_e) => {
                            
                        },
                    };
                }
                
                
                // if the event queue is not empty...
                while self.event_queue.len() != 0 {
                    // ...pop the next element that comes
                    if let Some(event) = self.event_queue.pop() {
                        match &event.event_type {
                            // and if categorize it based on entries
                            EventType::Background(ae) => {
                                match &ae.action {
                                    // simple bg change
                                    Action::ChgBgImg(_) => {
                                        self.screen_buffer.background = event.clone();
                                    },
                                    // adds tinting
                                    Action::ChgColImageColor(target_color) => {
                                        let rgb_hex = &target_color.to_owned()[0..7];
                                        let alpha_hex = u32::from_str_radix(&target_color.to_owned()[7..9], 16).unwrap() as f32;
                                        let rgba_color = colorsys::Rgb::from_hex_str(rgb_hex).unwrap();
                                        let color = speedy2d::color::Color::from_rgba(
                                            (rgba_color.red()/255.0) as f32,
                                            (rgba_color.green()/255.0) as f32,
                                            (rgba_color.blue()/255.0) as f32,
                                            (alpha_hex/255.0) as f32);
                                        
                                        self.screen_buffer.tint = color;
                                    }
                                    _ => ()
                                }
                            },
                            EventType::Text(_) => {
                                self.text_buffer.text_events.push(event);
                                
                            }
                            _ => ()
                        }
                         
                    }
                }
    
                // draw everything in screen buffer
                self.draw_screen_buffer(helper, graphics);
                if self.text_buffer.text_events.len() > 0 { self.draw_text_buffer(graphics) };
                // if diagnostics are turned on, draw them
                if self.diag.0 {
                    graphics.draw_text((0.0, 0.0), speedy2d::color::Color::RED, &text);
                }
            }
            
            helper.request_redraw();
    
            self.diag.1.draw_time = draw_start.elapsed().as_secs_f32() * 1000.0 * 1000.0;
            self.diag.1.counter += 1;
            self.diag.1.frame_count += 1;
            self.diag.1.fps = 1.0 / (draw_start - self.diag.1.last_update).as_secs_f32();
            self.diag.1.last_update = draw_start;
            //if screen_changed {helper.request_redraw()};
        }
    
        fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, _button: speedy2d::window::MouseButton) {
            self.play_pause();
        }
    
    
        fn on_keyboard_char(&mut self, helper: &mut WindowHelper<()>, unicode_codepoint: char) {
            dbg!(unicode_codepoint);
            match unicode_codepoint {
                'k' => self.change_track(),
                'p' => self.play_pause(),
                'f' => {
                    helper.set_fullscreen_mode(speedy2d::window::WindowFullscreenMode::FullscreenBorderless)
                },
                '\u{1b}' => {
                    helper.set_fullscreen_mode(speedy2d::window::WindowFullscreenMode::Windowed)
                }
                _ => ()
            }
        }

        fn on_key_down(
                &mut self,
                _helper: &mut WindowHelper<()>,
                virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
                _scancode: speedy2d::window::KeyScancode
            ) {
                if let Some(key) = virtual_key_code {
                    dbg!(&key);
                    match key {
                        speedy2d::window::VirtualKeyCode::Right => {
                            self.forward();
                        },
                        speedy2d::window::VirtualKeyCode::Left => {
                            self.backward();
                        },
                        _ => ()
                    }
                }
                
        }
    
        
        fn on_mouse_wheel_scroll(&mut self, _helper: &mut WindowHelper<()>, distance: speedy2d::window::MouseScrollDistance) {
            match distance {
                speedy2d::window::MouseScrollDistance::Lines { x: _, y, z: _ } => {
                    if y < 0.0 {
                        self.sender.send("VOL_DOWN".to_owned()).unwrap();
                    }
                    if y > 0.0 {
                        self.sender.send("VOL_UP".to_owned()).unwrap();
                    }
                },
                _ => ()
            }
         }
    }
}