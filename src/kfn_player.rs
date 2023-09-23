use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::Font;
use speedy2d::image::{ImageDataType, ImageSmoothingMode};
use speedy2d::shape::Rectangle;
use speedy2d::window::WindowHelper;
use speedy2d::Graphics2D;



use crate::helpers::Entry;
use crate::helpers::event::{Event, EventType};
use crate::kfn_data::KfnData;
use crate::kfn_ini::eff::Action;

mod window_handler;
mod text_buffer;
mod user_interactions;

use text_buffer::TextBuffer;

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
    text_buffer_vec: Vec<TextBuffer>,
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
        let diag = (
            true, 
            Diagnostics {
                counter: 0,
                frame_count: 0,
                last_update: std::time::Instant::now(),
                // TODO make this ship with the binary, and not be Linux dependent
                font: Font::new(include_bytes!(
                    "./fonts/NotoSansJP-Regular.ttf"
                ))
                .unwrap(),
                fps: 0.0,
                draw_time: 0.0,
            }
        );
        Self { 
            data,
            window_size: Vector2::from((window_size.0, window_size.1)),
            curr_background_entry: Entry::default(),
            _event_list: event_list,
            event_queue: Vec::new(),
            screen_buffer: ScreenBuffer { 
                background: Event::default(),
                tint: speedy2d::color::Color::WHITE,
                buffered_image: (String::new(), Vec::new()),
                resized: false 
            },
            text_buffer_vec: Vec::new(),
            time: TimeKeeper { 
                start_time: std::time::Instant::now(),
                offset: std::time::Duration::from_millis(0)
            },
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

    }

    
    /// Setting the initial state of the player to the parameters that are in the Songs.ini file.
    /// Sets the background, text color and font.
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

        for eff_num in 0..self.data.song.effs.len() {
        //for eff in &mut self.data.song.effs.clone() {

            let eff = self.data.song.effs[eff_num].clone();

            let eff_id = eff.id;

            if eff_id == 51 {
                continue;
            }


            
            let mut current_buffer = TextBuffer {
                eff_num,
                text_events: Vec::new(),
                font: Font::new(include_bytes!("fonts/NotoSansJP-Regular.ttf")).unwrap(),
                font_size: 70.0,
                inactive_color: speedy2d::color::Color::WHITE,
                inactive_outline_color: speedy2d::color::Color::BLACK,
                active_color: speedy2d::color::Color::YELLOW,
                active_outline_color: speedy2d::color::Color::BLACK,
                outline_weight: 5,
            };

            

            if let Some(inactive_color) = &self.data.song.effs[eff_num].initial_inactive_color {
                let s: Vec<String> = inactive_color.to_owned().trim().split("").map(|s| s.to_string()).collect();
                let r = u8::from_str_radix(&(s[2].clone() + &s[3]).to_ascii_lowercase(), 16).unwrap();
                let g = u8::from_str_radix(&(s[4].clone() + &s[5]).to_ascii_lowercase(), 16).unwrap();
                let b = u8::from_str_radix(&(s[6].clone() + &s[7]).to_ascii_lowercase(), 16).unwrap();
                let a = u8::from_str_radix(&(s[8].clone() + &s[9]).to_ascii_lowercase(), 16).unwrap();
                let hex = speedy2d::color::Color::from_int_rgba(r, g, b, a);
                current_buffer.inactive_color = hex;
            }
    
            if let Some(active_color) = &self.data.song.effs[eff_num].initial_active_color {
                let s: Vec<String> = active_color.to_owned().trim().split("").map(|s| s.to_string()).collect();
                let r = u8::from_str_radix(&(s[2].clone() + &s[3]).to_ascii_lowercase(), 16).unwrap();
                let g = u8::from_str_radix(&(s[4].clone() + &s[5]).to_ascii_lowercase(), 16).unwrap();
                let b = u8::from_str_radix(&(s[6].clone() + &s[7]).to_ascii_lowercase(), 16).unwrap();
                let a = u8::from_str_radix(&(s[8].clone() + &s[9]).to_ascii_lowercase(), 16).unwrap();
                let hex = speedy2d::color::Color::from_int_rgba(r, g, b, a);
                current_buffer.active_color = hex;
            }
    
            if let Some(font) = &self.data.song.effs[eff_num].initial_font {
                dbg!(&font.0);
                //dbg!(&self.text_buffer);
                current_buffer.font = Font::new(&self.data.get_entry_by_name(&font.0).unwrap().file_bin).unwrap();
                //dbg!(&self.text_buffer[n-1].font);
            }

            self.text_buffer_vec.push(current_buffer);
        }

        

    }
}
