



use std::time::Instant;
use std::thread;


use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use crossbeam::channel::unbounded;
use image::DynamicImage;
use image::imageops::FilterType;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::image::{ImageDataType, ImageSmoothingMode};
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::Graphics2D;



use crate::fonts::DefaultFonts;
use crate::helpers::Entry;
use crate::helpers::event::{Event, EventType};
use crate::kfn_data::KfnData;
use crate::kfn_ini::eff::Action;
use crate::kfn_ini::eff::AnimEntry;

#[derive(Debug, Clone)]
pub struct KfnPlayer {
    pub data: KfnData,
    pub window_size: Vector2<u32>,
    curr_background_entry: Entry,
    event_list: Vec<Event>,
    event_queue: Vec<Event>,
    screen_buffer: ScreenBuffer,
    receiver: Receiver<usize>,
    sender: Sender<String>,
    paused: bool,
}

#[derive(Debug, Clone)]
struct ScreenBuffer {
    background: Event,
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
    pub fn new(data: KfnData, window_size: (u32, u32), event_list: Vec<Event>, receiver: Receiver<usize>, sender: Sender<String>) -> Self {
        
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

    fn on_start(&mut self, helper: &mut WindowHelper<()>, info: WindowStartupInfo)  {
        helper.set_resizable(true);
        helper.set_icon_from_rgba_pixels(
            image::open("src/icons/icon32x32.png").unwrap().into_bytes(), (32, 32)).unwrap();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        //println!("Screen redrawn.");
        
        if !self.paused {

            // clear screen
            graphics.clear_screen(Color::BLACK);

            // draw everything in screen buffer
            self.draw_screen_buffer(graphics);

            // look for incoming events
            match self.receiver.try_recv() {
                Ok(event_time) => {
                    println!("{} received", event_time);
                    if let Some(next_event) = self.event_list.iter().position(|event| event.time  == event_time) {
                        let event = self.event_list[next_event].clone();
                        self.event_queue.push(event);
                        
                    } else {
                        
                    }
                    
                },
                Err(_e) => {
                    
                },
            };
            
            if self.event_queue.len() != 0 {
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
        
            /* if let Some(next_event) = self.event_buffer.first() {
                if next_event.time == self.receiver.try_recv().unwrap_or(0) {
                    match &next_event.event_type {
                        EventType::Animation(ae) => {
                            dbg!(&self.start_time.elapsed().as_millis());
                            match &ae.action {
                                crate::kfn_ini::eff::Action::ChgBgImg(entryname) => {
                                    dbg!(&entryname);
                                    self.set_background(&entryname, graphics);
                                },
                                _ => ()
                            }
                        },
                        EventType::Text(t) => {

                        }
                    }
                }
            }*/

            //self.set_background(&self.data.song.effs[0].initial_lib_image, graphics);
            

            
        }

        helper.request_redraw();
        //if screen_changed {helper.request_redraw()};
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, _button: speedy2d::window::MouseButton) {
            self.play_pause();
    }
}