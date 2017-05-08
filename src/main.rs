extern crate image;

mod parameters;
mod image_read_writer;
mod packer;
mod stop_watch;

use std::env;
use parameters::ResizeMode;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_path = args[2].parse().unwrap();

    let mut images = image_read_writer::get_all_images(&target_path);

    let params = parameters::Parameters {
        alpha_threshold: 100,
        out_texture_size: (1024, 1024),
        padding: 20,
        sprite_sheet_size_x: 2,
        sprite_sheet_size_y: 2,
        resize_mode: ResizeMode::NoKeepAspectRatio,
    };

    let packed_image = packer::pack(images.as_mut_slice(), &params);

    image_read_writer::save_image(&packed_image, &target_path);
}
