pub struct DefaultFonts;

impl DefaultFonts {
    pub fn arial() -> &'static [u8]{
        include_bytes!("LiberationSans-Bold.ttf")
    }
}