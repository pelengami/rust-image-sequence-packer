extern crate image;

use std::time::SystemTime;
use std::env;

mod parameters;
mod image_read_writer;
mod packer;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!()
    }

    let target_path = args[1].parse().unwrap();

    let mut images = image_read_writer::get_all_images(&target_path);

    let params = parameters::Parameters {
        alpha_threshold: 20,
        output_texture_dimension: (512, 512),
        padding: 0,
        tiling_x: 2,
        tiling_y: 2,
    };

    let packed_image = packer::pack(images.as_mut_slice(), &params);

    image_read_writer::save_image(&packed_image, &target_path);

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Seconds: {}", sec);
}
