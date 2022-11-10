



use std::time::Instant;


use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use image::imageops::FilterType;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::image::{ImageDataType, ImageSmoothingMode};
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::Graphics2D;



use crate::helpers::Entry;
use crate::helpers::event::{Event, EventType};
use crate::kfn_data::KfnData;
use crate::kfn_ini::eff::Action;

#[derive(Debug, Clone)]
pub struct KfnPlayer {
    pub data: KfnData,
    pub window_size: Vector2<u32>,
    curr_background_entry: Entry,
    event_list: Vec<Event>,
    event_queue: Vec<Event>,
    screen_buffer: ScreenBuffer,
    receiver: Receiver<Event>,
    sender: Sender<String>,
    paused: bool,
    diag: (bool, Diagnostics),
}

#[derive(Debug, Clone)]
struct ScreenBuffer {
    background: Event,
}

#[derive(Debug, Clone)]
struct Diagnostics {
    counter: usize,
    frame_count: u32,
    last_update: Instant,
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
    pub fn new(data: KfnData, window_size: (u32, u32), event_list: Vec<Event>, receiver: Receiver<Event>, sender: Sender<String>) -> Self {
        let diag = (true, Diagnostics {
            counter: 0,
            frame_count: 0,
            last_update: std::time::Instant::now(),
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
            event_list,
            event_queue: Vec::new(),
            screen_buffer: ScreenBuffer { background: Event::default() },
            receiver,
            sender,
            paused: false,
            diag,
        }
    }

    /// Function for setting the player's background.
    fn set_background(&self, entry_name: &str, graphics: &mut Graphics2D) {
        
        graphics.clear_screen(Color::BLACK);
        // match for initial image in library
        match self.data.get_entry_by_name(entry_name) {
            Some(background_entry) => {
                if background_entry != self.curr_background_entry {
                    //load raw image data from memory
                    let raw_image = image::load_from_memory(&background_entry.file_bin)
                    .expect("Invalid image file.")
                    // resize to fit the window
                    .resize_to_fill(self.window_size.x, self.window_size.y, FilterType::Triangle)
                    .into_rgb8().into_raw();
                    // convert raw image data to an actual drawable image
                    let image = graphics.create_image_from_raw_pixels(
                    ImageDataType::RGB, 
                    ImageSmoothingMode::NearestNeighbor,
                    // scale for current window
                    (self.window_size.x, self.window_size.y),
                    &raw_image).unwrap();
                    graphics.draw_image(Vector2::new(0.0, 0.0), &image);
                }
            },
            None => {
                println!("Image entry {} not found.", entry_name);
            },
        }
    }

    fn draw_screen_buffer(&self, graphics: &mut Graphics2D) {
        let bg = self.screen_buffer.background.event_type.clone();
        match bg {
            EventType::Animation(ae) => {
                match ae.action {
                    Action::ChgBgImg(bg_entry) => {
                        self.set_background(&bg_entry, graphics)
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }
    /// Function for pausing and resuming the sink thread.
    fn play_pause(&mut self) {
        if self.paused {
            self.sender.send("RESUME".to_string()).unwrap();
            self.paused = false;
            println!("KFN-PLAYER: RESUME signal sent.")
        } else {
            self.sender.send("PAUSE".to_string()).unwrap();
            self.paused = true;
            println!("KFN-PLAYER: PAUSE signal sent.")
        }
    }
    
}


impl WindowHandler for KfnPlayer {

    fn on_start(&mut self, helper: &mut WindowHelper<()>, _info: WindowStartupInfo)  {
        helper.set_resizable(true);
        helper.set_icon_from_rgba_pixels(
            image::open("src/icons/icon32x32.png").unwrap().into_bytes(), (32, 32)).unwrap();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        //println!("Screen redrawn.");
        let draw_start = Instant::now();
        let text = 
            &self.diag.1.font.layout_text(
                &std::format!(
                    "Frame: {}, FPS: {:.2}, frame draw time: {:.2} µs",
                    self.diag.1.counter,
                    self.diag.1.fps,
                    self.diag.1.draw_time),
                    42.0,
                    TextOptions::new());
        

        if !self.paused {

            // clear screen
            graphics.clear_screen(Color::BLACK);

            // draw everything in screen buffer
            self.draw_screen_buffer(graphics);

            // look for incoming events
            match self.receiver.try_recv() {
                Ok(event_recv) => {
                    println!("{} received", event_recv.time);
                    
                        self.event_queue.push(event_recv);
                        

                },
                Err(_e) => {
                    
                },
            };
            
            while self.event_queue.len() != 0 {
                if let Some(event) = self.event_queue.pop() {
                    match &event.event_type {
                        EventType::Animation(ae) => {
                            match &ae.action {
                                Action::ChgBgImg(_) => {
                                    self.screen_buffer.background = event.clone();
                                },
                                _ => ()
                            }
                            
                        },
                        _ => ()
                    }
                    
                }
            }
            

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
}