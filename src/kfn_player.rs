use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::image::{ImageDataType, ImageSmoothingMode, ImageHandle};
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::Graphics2D;



use crate::helpers::Entry;
use crate::helpers::event::{Event, EventType};
use crate::kfn_data::KfnData;
use crate::kfn_ini::eff::Action;

/// The windowed graphical player of the kfn-rs library.
#[derive(Debug, Clone)]
pub struct KfnPlayer {
    /// The data a parsed the .kfn file.
    pub data: KfnData,
    /// The size of the window.
    pub window_size: Vector2<u32>,
    curr_background_entry: Entry,
    _event_list: Vec<Event>,
    event_queue: Vec<Event>,
    screen_buffer: ScreenBuffer,
    text_buffer: TextBuffer,
    time: TimeKeeper,
    receiver: crossbeam::channel::Receiver<Event>,
    sender: crossbeam::channel::Sender<String>,
    paused: bool,
    diag: (bool, Diagnostics),
}

#[derive(Debug, Clone)]
struct ScreenBuffer {
    background: Event,
    tint: Color,
    buffered_image: (String, Vec<u8>),
    resized: bool
}

#[derive(Debug, Clone)]
struct TextBuffer {
    text_events: Vec<Event>,
    font: Font,
    color: speedy2d::color::Color
}

#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time: std::time::Instant,
    offset: std::time::Duration,
}

/// Container for diagnostics data
#[derive(Debug, Clone)]
struct Diagnostics {
    counter: usize,
    frame_count: u32,
    last_update: std::time::Instant,
    fps: f32,
    draw_time: f32,
    font: Font,
}

impl KfnPlayer {
    /// Returns a KfnPlayer, which can visualize the parsed KfnData
    /// 
    /// # Arguments
    /// * `data` - The parsed data from a Kfn file
    /// * `window_size` - The size of the player window
    /// * `event_list` - The list of events to be played back by the KfnPlayer
    /// * `receiver` - The timing signal coming from the thread in the kfn-rs library
    /// 
    pub fn new(data: KfnData, window_size: (u32, u32), event_list: Vec<Event>, receiver: crossbeam::channel::Receiver<Event>, sender: crossbeam::channel::Sender<String>) -> Self {
        let diag = (true, Diagnostics {
            counter: 0,
            frame_count: 0,
            last_update: std::time::Instant::now(),
            // TODO make this ship with the binary, and not be Linux dependent
            font: Font::new(include_bytes!(
                "/usr/share/fonts/noto/NotoSans-Regular.ttf"
            ))
            .unwrap(),
            fps: 0.0,
            draw_time: 0.0,
        });

        Self { 
            data,
            window_size: Vector2::from((window_size.0, window_size.1)),
            curr_background_entry: Entry::default(),
            _event_list: event_list,
            event_queue: Vec::new(),
            screen_buffer: ScreenBuffer { background: Event::default(), tint: speedy2d::color::Color::WHITE, buffered_image: (String::new(), Vec::new()), resized: false },
            text_buffer: TextBuffer {
                text_events: Vec::new(),
                font: Font::new(include_bytes!("/usr/share/fonts/noto/NotoSans-Regular.ttf")).unwrap(),
                color: speedy2d::color::Color::WHITE,
            },
            time: TimeKeeper { start_time: std::time::Instant::now(), offset: std::time::Duration::from_millis(0) },
            receiver,
            sender,
            paused: false,
            diag,
        }
    }

    /// Function for setting the player's background.
    fn set_background(&mut self, entry_name: &str, graphics: &mut Graphics2D) {
        
        graphics.clear_screen(Color::BLACK);
        // match for initial image in library
        match self.data.get_entry_by_name(entry_name) {
            Some(background_entry) => {
                if background_entry != self.curr_background_entry {
                    //load raw image data from memory
                    
                    // if the image is the same, and the window has not been resized, only then should we
                    // also rebuild the image from memory
                    if &background_entry.filename != &self.screen_buffer.buffered_image.0 || self.screen_buffer.resized {
                        self.screen_buffer.resized = false;

                        // resize the image itself, and save it
                        self.screen_buffer.buffered_image.0 = background_entry.filename;
                        self.screen_buffer.buffered_image.1 = image::load_from_memory(&background_entry.file_bin)
                        .expect("Invalid image file.")
                        // resize to fit the window
                        .resize_to_fill(self.window_size.x, self.window_size.y, image::imageops::FilterType::Triangle)
                        .into_rgb8().into_raw();
                    }

                    // convert raw image data to an actual drawable image
                    let image = graphics.create_image_from_raw_pixels(
                    ImageDataType::RGB, 
                    ImageSmoothingMode::NearestNeighbor,
                    // scale for current window
                    (self.window_size.x, self.window_size.y),
                    &self.screen_buffer.buffered_image.1).unwrap();
                    
                    let rect = Rectangle::new(
                        Vector2::new(0.0, 0.0),
                        Vector2::new(self.window_size.x as f32, self.window_size.y as f32),
                    );
                    graphics.draw_rectangle_image_tinted(rect, self.screen_buffer.tint, &image);
                    //graphics.draw_image(Vector2::new(0.0, 0.0), &image);
                }
            },
            None => {
                //println!("Image entry {} not found.", entry_name);
            },
        }
    }

    fn draw_text_buffer(&mut self, graphics: &mut Graphics2D) {
        
        if (self.text_buffer.text_events[self.text_buffer.text_events.len()-2].time * 10) as u128 <= (self.time.offset + self.time.start_time.elapsed()).as_millis()  {
            self.text_buffer.text_events.pop();
        }
        let text = match &self.text_buffer.text_events[self.text_buffer.text_events.len()-1].event_type {
            EventType::Text(s) => Into::<String>::into(s.to_owned()),
            _ => "".to_string()
        };
        let ftext = self.text_buffer.font.layout_text(&text, 50.0, TextOptions::new());
        graphics.draw_text((200.0, 200.0), self.text_buffer.color, &ftext);
    }

    fn draw_screen_buffer(&mut self, _helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        let bg = self.screen_buffer.background.event_type.clone();
        //for event in self.screen_buffer.clone() {
            match bg {
                EventType::Background(ae) => {
                    match ae.action {
                        Action::ChgBgImg(bg_entry) => {
                            self.set_background(&bg_entry, graphics);
                        }
                        
                        _ => ()
                    }
                }
                _ => ()
            }
        
        //let bg = self.screen_buffer.background.event_type.clone();
        //match bg {
            
        //}
    }
    /// Function for pausing and resuming the sink thread.
    fn play_pause(&mut self) {
        if self.paused {
            self.time.start_time = std::time::Instant::now();
            self.sender.send("RESUME".to_string()).unwrap();
            self.paused = false;
            println!("KFN-PLAYER: RESUME signal sent.")
        } else {
            self.time.offset = self.time.start_time.elapsed() + self.time.offset;
            self.sender.send("PAUSE".to_string()).unwrap();
            self.paused = true;
            println!("KFN-PLAYER: PAUSE signal sent.")
        }
    }

    fn change_track(&mut self) {
        self.sender.send("CH_TRACK".to_string()).unwrap();
    }
    
    /// Setting the initial state of the player.
    fn set_initial_state(&mut self) {
        // initial bg
        if let Some(initial_bg) = self.data.song.effs[0].initial_lib_image.clone() {
            self.event_queue.push(Event {
                time: 0,
                event_type: EventType::Background(
                        crate::kfn_ini::eff::AnimEntry {
                            action: Action::ChgBgImg(initial_bg),
                            effect: None, trans_time: 0.0,
                            trans_type: crate::kfn_ini::eff::TransType::None,
                        }
                    )
            })
        }

        if let Some(inactive_color) = &self.data.song.effs[1].initial_inactive_color {
            let s: Vec<String> = inactive_color.to_owned().trim().split("").map(|s| s.to_string()).collect();
            let r = u8::from_str_radix(&(s[2].clone() + &s[3]).to_ascii_lowercase(), 16).unwrap();
            let g = u8::from_str_radix(&(s[4].clone() + &s[5]).to_ascii_lowercase(), 16).unwrap();
            let b = u8::from_str_radix(&(s[6].clone() + &s[7]).to_ascii_lowercase(), 16).unwrap();
            let a = u8::from_str_radix(&(s[8].clone() + &s[9]).to_ascii_lowercase(), 16).unwrap();
            let hex = speedy2d::color::Color::from_int_rgba(r, g, b, a);
            self.text_buffer.color = hex;
        }

        if let Some(font) = &self.data.song.effs[1].initial_font {
            dbg!(&font.0);
            self.text_buffer.font = Font::new(&self.data.get_entry_by_name(&font.0).unwrap().file_bin).unwrap();
            dbg!(&self.text_buffer.font);
        }

    }
}


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


    fn on_keyboard_char(&mut self, _helper: &mut WindowHelper<()>, unicode_codepoint: char) {
        match unicode_codepoint {
            'k' => self.change_track(),
            'p' => self.play_pause(),
            _ => ()
        }
    }

    
    fn on_mouse_wheel_scroll(&mut self, helper: &mut WindowHelper<()>, distance: speedy2d::window::MouseScrollDistance) {
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