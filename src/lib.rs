#![crate_type = "lib"]

extern crate image;

pub mod parameters;
pub mod resize_mode;
pub mod image_read_writer;
pub mod packer;
pub mod stop_watch;

use image::DynamicImage;

pub fn pack(images_path: &[String], output_image_path: &String, alpha_threshold: u8, out_texture_width: u32, out_texture_height: u32, padding: u32, size_x: u32, size_y: u32) {
    let params = parameters::Parameters {
        alpha_threshold: alpha_threshold,
        out_texture_size: (out_texture_width, out_texture_height),
        padding: padding,
        columns: size_x,
        rows: size_y,
        resize_mode: resize_mode::ResizeMode::NoKeepAspectRatio,
    };

    let mut images = vec!();

    for path in images_path {
        let dynamic_image = image_read_writer::open_image(&path);
        images.push(dynamic_image);
    }

    let packed_image = packer::pack(images.as_mut_slice(), &params);

    packed_image.save(output_image_path);
}
