use std::path::{PathBuf, Path};

use image::{DynamicImage, GenericImageView};

use cells::CELL_SIZE;

pub mod cells;

pub fn get_file_paths(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let files_iter = std::fs::read_dir(&dir).unwrap();
    for file in files_iter {
        if let Ok(file) = file {
            if file.file_type().unwrap().is_file() {
                paths.push(file.path());
            }
        }
    }
    paths
}

pub fn average_image_color(image: &DynamicImage) -> (u8, u8, u8) {
    let mut r: u64 = 0;
    let mut g: u64 = 0;
    let mut b: u64 = 0;
    let mut count: u64 = 0;

    for (_, _, color) in image.pixels() {
        r += color.0[0] as u64;
        g += color.0[1] as u64;
        b += color.0[2] as u64;
        count += 1;
    }

    r /= count;
    g /= count;
    b /= count;

    (r as u8, g as u8, b as u8)
}

fn crop_image(image: &mut DynamicImage) -> DynamicImage {
    let image_width = image.width();
    let image_height = image.height();
    let cropped_width = image_width - (image_width % CELL_SIZE);
    let cropped_height = image_height - (image_height % CELL_SIZE);
    image.crop(0, 0, cropped_width, cropped_height)
}