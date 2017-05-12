use resize_mode::ResizeMode;

pub struct Parameters {
    pub alpha_threshold: u8,
    pub out_texture_size: (u32, u32),
    pub padding: u32,
    pub columns: u32,
    pub rows: u32,
    pub resize_mode: ResizeMode
}
