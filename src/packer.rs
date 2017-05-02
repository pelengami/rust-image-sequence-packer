use image;
use image::GenericImage;
use image::ImageBuffer;
use image::DynamicImage;
use image::imageops;
use parameters::Parameters;
use std::cmp;

pub fn pack(images: &mut [image::DynamicImage], params: &Parameters) -> image::RgbaImage {
    let (min_x, min_y, max_x, max_y) = fin_bound_box_all(&images, &params);

    let (out_texture_width, out_texture_height) = params.output_texture_dimension;

    let new_width = out_texture_width / params.tiling_x;
    let new_height = out_texture_height / params.tiling_y;

    let mut out_texture: image::RgbaImage = ImageBuffer::new(out_texture_width, out_texture_height);

    let croped_images = crop_images(images, min_x, min_y, max_x, max_y);

    let mut hor_step = 0;
    let mut vert_step = 0;

    for img in &croped_images {
        let resized_img = img.resize(new_width, new_height, imageops::FilterType::CatmullRom);

        let (resized_img_width, resized_img_height) = resized_img.dimensions();

        let hor_padding = new_width - resized_img_width;
        let vert_padding = new_height - resized_img_height;

        for i in hor_padding / 2..resized_img_width {
            for j in vert_padding / 2..resized_img_height {
                let pixel = resized_img.get_pixel(i, j);
                out_texture.put_pixel(hor_step + i, vert_step + j, pixel);
            }
        }

        hor_step += resized_img_width + hor_padding;
        if hor_step + resized_img_width + hor_padding > out_texture_width {
            hor_step = 0;
            vert_step += resized_img_height + vert_padding;
        }
    }

    out_texture
}

fn fin_bound_box_all(images: &[image::DynamicImage], params: &Parameters) -> (u32, u32, u32, u32) {
    let mut min_x: u32 = <u32>::max_value();
    let mut max_x: u32 = 0;

    let mut min_y: u32 = <u32>::max_value();
    let mut max_y: u32 = 0;

    for img in images {
        let (temp_min_x, temp_min_y, temp_max_x, temp_max_y) = find_bound_box(img, &params);

        min_x = cmp::min(min_x, temp_min_x);
        min_y = cmp::min(min_y, temp_min_y);
        max_x = cmp::max(max_x, temp_max_x);
        max_y = cmp::max(max_y, temp_max_y);
    }

    (min_x - params.padding, min_y - params.padding, max_x + params.padding, max_y + params.padding)
}

fn crop_images(images: &mut [image::DynamicImage], min_x: u32, min_y: u32, max_x: u32, max_y: u32) -> Vec<image::DynamicImage> {
    let mut sub_images = vec!();

    for img in images {
        let sub_img = img.crop(min_x, min_y, max_x - min_x, max_y - min_y);
        sub_images.push(sub_img);
    }

    sub_images
}

fn find_bound_box(image: &image::DynamicImage, params: &Parameters) -> (u32, u32, u32, u32) {
    let mut min_x: u32 = image.dimensions().0;
    let mut max_x: u32 = 0;

    let mut min_y: u32 = image.dimensions().1;
    let mut max_y: u32 = 0;

    let (width, height) = image.dimensions();

    for i in 0..width {
        for j in 0..height {
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
