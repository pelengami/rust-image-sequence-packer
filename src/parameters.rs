pub struct Parameters {
    pub alpha_threshold: u8,
    pub out_texture_size: (u32, u32),
    pub padding: u32,
    pub sprite_sheet_size_x: u32,
    pub sprite_sheet_size_y: u32,
    pub resize_mode: ResizeMode
}

pub enum ResizeMode {
    KeepAspectRatio,
    NoKeepAspectRatio
}