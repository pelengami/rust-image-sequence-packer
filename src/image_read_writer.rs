use image;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::ops::Add;

pub fn get_all_images(directoryPath: &String) -> Vec<image::DynamicImage> {
    let images_path = get_images_paths(directoryPath);

    let mut images = vec!();

    for image_path in images_path {
        let img = image::open(&Path::new(&image_path)).unwrap();
        images.push(img);
    }

    images
}

pub fn save_image(image: &image::RgbaImage, path: &String) {
    let file_name: String = "out_texture.png".to_string();
    let save_path = path.clone().add("\\").add(&file_name);
    let _ = image.save(&save_path).unwrap();
}

fn get_images_paths(directoryPath: &String) -> Vec<String> {
    let mut res_paths = vec!();

    let paths = fs::read_dir(directoryPath).unwrap();

    for img_path in paths {
        let path = img_path.unwrap().path();

        let extension = {
            let extension = path.extension();
            if extension != Some(OsStr::new("png")) {
                continue;
            }
        };

        let str_path = path.into_os_string().into_string();
        match str_path {
            Ok(str_path) => {
                res_paths.push(str_path);
            }
            _ => {}
        }
    }

    res_paths
}
