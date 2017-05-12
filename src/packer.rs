use image;
use image::GenericImage;
use image::ImageBuffer;
use image::DynamicImage;
use image::imageops;
use parameters::Parameters;
use resize_mode::ResizeMode;
use std::cmp;

pub fn pack(images: &mut [image::DynamicImage], params: &Parameters) -> image::RgbaImage {
    let (min_x, min_y, max_x, max_y) = calc_trim_size_all(&images, &params);

    let (out_texture_width, out_texture_height) = params.out_texture_size;

    let each_texture_width = out_texture_width / params.columns;
    let each_texture_height = out_texture_height / params.rows;

    let croped_images = crop_images(images, min_x, min_y, max_x, max_y);

    let packed_texture = pack_images(each_texture_width, each_texture_height, out_texture_width, out_texture_height, &croped_images, params.padding, &params.resize_mode);

    packed_texture
}

fn pack_images(each_texture_width: u32, each_texture_height: u32, out_texture_width: u32, out_texture_height: u32, croped_images: &[image::DynamicImage], padding: u32, resize_mode: &ResizeMode) -> image::RgbaImage {
    let mut out_texture: image::RgbaImage = ImageBuffer::new(out_texture_width, out_texture_height);

    let mut h_step = padding;
    let mut v_step = padding;

    for image in croped_images {
        let resized_image = resize_image(each_texture_width - padding * 2, each_texture_height - padding * 2, image, resize_mode);
        let (resized_img_width, resized_img_height) = resized_image.dimensions();

        let h_padding: u32;
        let v_padding: u32;

        match *resize_mode {
            ResizeMode::KeepAspectRatio => {
                h_padding = each_texture_width - resized_img_width;
                v_padding = each_texture_height - resized_img_height;
            }
            ResizeMode::NoKeepAspectRatio => {
                h_padding = padding;
                v_padding = padding;
            }
        }

        for j in 0..resized_img_height {
            for i in 0..resized_img_width {
                let pixel = resized_image.get_pixel(i, j);
                out_texture.put_pixel(h_step + i, v_step + j, pixel);
            }
        }

        h_step += resized_img_width + h_padding * 2;
        if h_step + resized_img_width + h_padding > out_texture_width {
            h_step = h_padding;
            v_step += resized_img_height + v_padding * 2;
        }
    }

    out_texture
}

fn resize_image(each_texture_width: u32, each_texture_height: u32, image: &DynamicImage, resize_mode: &ResizeMode) -> image::DynamicImage {
    let mut resized_image: DynamicImage;

    match *resize_mode {
        ResizeMode::KeepAspectRatio => {
            resized_image = image.resize(each_texture_width, each_texture_height, imageops::FilterType::Lanczos3);
        }
        ResizeMode::NoKeepAspectRatio => {
            resized_image = image.resize_exact(each_texture_width, each_texture_height, imageops::FilterType::Lanczos3);
        }
    }

    resized_image
}

fn calc_trim_size_all(images: &[image::DynamicImage], params: &Parameters) -> (u32, u32, u32, u32) {
    let mut min_x: u32 = <u32>::max_value();
    let mut max_x: u32 = 0;

    let mut min_y: u32 = <u32>::max_value();
    let mut max_y: u32 = 0;

    for img in images {
        let (temp_min_x, temp_min_y, temp_max_x, temp_max_y) = calc_trim_size_alpha(img, &params);

        min_x = cmp::min(min_x, temp_min_x);
        min_y = cmp::min(min_y, temp_min_y);
        max_x = cmp::max(max_x, temp_max_x);
        max_y = cmp::max(max_y, temp_max_y);
    }

    (min_x, min_y, max_x, max_y)
}

fn crop_images(images: &mut [image::DynamicImage], min_x: u32, min_y: u32, max_x: u32, max_y: u32) -> Vec<image::DynamicImage> {
    let mut sub_images = vec!();

    for img in images {
        let sub_img = img.crop(min_x, min_y, max_x - min_x, max_y - min_y);
        sub_images.push(sub_img);
    }

    sub_images
}

fn calc_trim_size_alpha(image: &image::DynamicImage, params: &Parameters) -> (u32, u32, u32, u32) {
    let mut min_x: u32 = image.dimensions().0;
    let mut max_x: u32 = 0;

    let mut min_y: u32 = image.dimensions().1;
    let mut max_y: u32 = 0;

    let (width, height) = image.dimensions();

    for j in 0..height {
        for i in 0..width {
            let pixel = image.get_pixel(i, j);
            let alpha = pixel[3];

            if alpha > params.alpha_threshold {
                min_x = cmp::min(min_x, i);
                min_y = cmp::min(min_y, j);
                max_x = cmp::max(max_x, i);
                max_y = cmp::max(max_y, j);
            }
        }
    }

    (min_x, min_y, max_x, max_y)
}
