

use crossbeam::channel::Receiver;

use image::imageops::FilterType;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::{Font, TextLayout, TextOptions};

use speedy2d::image::{ImageDataType, ImageSmoothingMode};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;



use crate::kfn_data::KfnData;

#[derive(Debug, Clone)]
pub struct KfnPlayer {
    pub data: KfnData,
    pub curr_window_size: Vector2<u32>,
    pub receiver: Receiver<String>,
}

impl KfnPlayer {
    pub fn new(data: KfnData, curr_window_size: (u32, u32), receiver: Receiver<String>) -> Self {
        Self { data, curr_window_size: Vector2::from((curr_window_size.0, curr_window_size.1)), receiver }
    }
}


impl WindowHandler for KfnPlayer {

    fn on_resize(
            &mut self,
            helper: &mut WindowHelper<()>,
            size_pixels: Vector2<u32>
        ) {
        
        self.curr_window_size = size_pixels;
        helper.request_redraw()

    }


    fn on_draw(
            &mut self,
            helper: &mut WindowHelper<()>,
            graphics: &mut Graphics2D
        ) {

            graphics.clear_screen(Color::BLACK);

        
        
        match self.data.get_entry_by_name(
                &self.data.song.effs[0].initial_lib_image
        ) {
            Some(f) => {
                let mut image = image::load_from_memory(&f.file_bin).unwrap();
                image = image.resize_exact(self.curr_window_size.x, self.curr_window_size.y, FilterType::Nearest);
                let xd = graphics.create_image_from_raw_pixels(ImageDataType::RGB, ImageSmoothingMode::NearestNeighbor, (self.curr_window_size.x, self.curr_window_size.y), &image.to_rgb8());
                graphics.draw_image(Vector2::new(0.0, 0.0), &xd.unwrap())
            },
            None => (),
        }

        match self.receiver.try_recv() {
            Ok(s) => {
                let scale = (self.data.song.effs[1].initial_font.1 / 3) as f32;
                let font = Font::new(&self.data.get_entry_by_name(&self.data.song.effs[1].initial_font.0).unwrap().file_bin).unwrap();
                let text = font.layout_text(&s, scale, TextOptions::new());
                graphics.draw_text(Vector2::from(((self.curr_window_size.x/2) as f32, (self.curr_window_size.y/2) as f32)), Color::BLUE, &text)
            },
            Err(_) => (),
        }
 
        helper.request_redraw();
    }
}