extern crate packer;

use std::env;

use packer::parameters::Parameters;
use packer::resize_mode::ResizeMode;
use packer::image_read_writer;
use packer::stop_watch::StopWatch;

fn main() {
    let mut stop_watch = StopWatch::new();
    stop_watch.start();

    let params = Parameters {
        alpha_threshold: 100,
        out_texture_size: (256, 256),
        padding: 20,
        columns: 2,
        rows: 1,
        resize_mode: ResizeMode::NoKeepAspectRatio,
    };

    let args: Vec<String> = env::args().collect();

    let images_path = vec![args[2].parse().unwrap(), args[3].parse().unwrap()];
    let output_image_path: String = args[4].parse().unwrap();

    let mut images = vec!();

    for path in images_path {
        let dynamic_image = image_read_writer::open_image(&path);
        images.push(dynamic_image);
    }

    let packed_image = packer::packer::pack(images.as_mut_slice(), &params);

    packed_image.save(output_image_path);

    stop_watch.stop();
    stop_watch.print();
}
