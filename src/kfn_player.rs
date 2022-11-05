



use std::time::Instant;


use crossbeam::channel::Receiver;

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

#[derive(Debug, Clone)]
pub struct KfnPlayer {
    pub data: KfnData,
    pub curr_window_size: Vector2<u32>,
    curr_background_entry: Entry,
    event_buffer: Vec<Event>,
    start_time: Instant,
    receiver: Receiver<usize>,

}

impl KfnPlayer {
    pub fn new(data: KfnData, curr_window_size: (u32, u32), event_buffer: Vec<Event>, receiver: Receiver<usize>) -> Self {
        Self { 
            data,
            curr_window_size: Vector2::from((curr_window_size.0, curr_window_size.1)),
            curr_background_entry: Entry::default(),
            event_buffer,
            start_time: Instant::now(),
            receiver
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
                    .resize_to_fill(self.curr_window_size.x, self.curr_window_size.y, FilterType::Triangle)
                    .into_rgb8().into_raw();
                    // convert raw image data to an actual drawable image
                    let image = graphics.create_image_from_raw_pixels(
                    ImageDataType::RGB, 
                    ImageSmoothingMode::NearestNeighbor,
                    // scale for current window
                    (self.curr_window_size.x, self.curr_window_size.y),
                    &raw_image).unwrap();
                    graphics.draw_image(Vector2::new(0.0, 0.0), &image);
                    println!("Background changed.");
                }
            },
            None => {
                println!("Image entry {} not found.", entry_name);
            },
        }
    }

    
}


impl WindowHandler for KfnPlayer {

    fn on_start(&mut self, helper: &mut WindowHelper<()>, info: WindowStartupInfo) {
        helper.set_resizable(false);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        let mut screen_changed = false;
        // clear screen
        //graphics.clear_screen(Color::BLACK); // need to implement default bg color
        //self.set_background("kaibutsu_05.jpg", graphics);
        loop {

       
            match self.receiver.try_recv() {
                Ok(event_time) => {
                    println!("{} received", event_time);
                    if let Some(next_event) = self.event_buffer.iter().position(|event| event.time  == event_time) {
                        match &self.event_buffer[next_event].event_type {
                            EventType::Animation(ae) => {
                                //dbg!(&self.start_time.elapsed().as_millis());
                                match &ae.action {
                                    crate::kfn_ini::eff::Action::ChgBgImg(entryname) => {
                                        dbg!(&entryname);
                                        
                                        self.set_background(&entryname, graphics);
                                        screen_changed = true;
                                        break;
                                    },
                                    _ => println!("Unimplemented AnimEntry Action! Skipping...")
                                }
                            },
                            EventType::Text(t) => {
                                print!("_");
                            }
                        }
                    }
                },
                Err(_e) => {
                
                },
            };
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
        

        
        if screen_changed {helper.request_redraw()};
    }
    
}