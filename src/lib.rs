#![crate_type = "lib"]

extern crate image;

mod parameters;
mod resize_mode;
mod image_read_writer;
mod packer;
mod stop_watch;

use image::DynamicImage;

#[no_mangle]
pub extern "C" fn pack(images_path: Vec<String>, alpha_threshold: u8, out_texture_width: u32, out_texture_height: u32, padding: u32, size_x: u32, size_y: u32, resize_mode: resize_mode::ResizeMode) -> Vec<u8> {
    let params = parameters::Parameters {
        alpha_threshold: alpha_threshold,
        out_texture_size: (out_texture_width, out_texture_height),
        padding: padding,
        sprite_sheet_size_x: size_x,
        sprite_sheet_size_y: size_y,
        resize_mode: resize_mode,
    };

    let mut images = vec!();

    for path in images_path {
        let dynamic_image = image_read_writer::open_image(&path);
        images.push(dynamic_image);
    }

    let packed_image = packer::pack(images.as_mut_slice(), &params);

    packed_image.into_vec()
}