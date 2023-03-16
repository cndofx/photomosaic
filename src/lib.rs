use image::DynamicImage;

use cells::CELL_SIZE;

pub mod cells;

fn crop_image(image: &mut DynamicImage) -> DynamicImage {
    let image_width = image.width();
    let image_height = image.height();
    let cropped_width = image_width - (image_width % CELL_SIZE);
    let cropped_height = image_height - (image_height % CELL_SIZE);
    image.crop(0, 0, cropped_width, cropped_height)
}