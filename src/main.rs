use image::{io::Reader as ImageReader, Rgb, RgbImage, RgbaImage, DynamicImage};
use rayon::prelude::*;

use photomosaic::{average_image_color, cells::Cells, get_file_paths, image_record::ImageRecord};


const OUTPUT_CELL_SIZE: u32 = 100;

fn main() {
    let mut img = ImageReader::open("img/input.png")
        .unwrap()
        .decode()
        .unwrap();

    let cells = Cells::from_image(&mut img);

    // convert cells back into an image
    let mut pixelated_image = RgbImage::new(cells.width(), cells.height());
    for (i, color) in cells.iter().enumerate() {
        let i = i as u32;
        let x = i % cells.width();
        let y = i / cells.width();

        let r = color.0 as u8;
        let g = color.1 as u8;
        let b = color.2 as u8;

        pixelated_image.put_pixel(x, y, Rgb([r, g, b]))
    }
    pixelated_image.save("img/pixelated.png").unwrap();

    // ========================

    let files = get_file_paths("img/sources/cropped/");

    let records: Vec<ImageRecord> = files.into_par_iter().map(|path| {
        let img = ImageReader::open(&path).unwrap().decode().unwrap();
        let color = average_image_color(&img);
        ImageRecord::new(path, color)
    }).collect();

    dbg!(&records);

    let result = cells_to_image(&cells, &records);
    result.save("img/output.png").unwrap();
}

fn cells_to_image(cells: &Cells, images: &[ImageRecord]) -> DynamicImage {
    let mut image = RgbaImage::new(cells.width() * OUTPUT_CELL_SIZE, cells.height() * OUTPUT_CELL_SIZE);

    for (i, color) in cells.iter().enumerate() {
        let r = color.0 as u8;
        let b = color.1 as u8;
        let g = color.2 as u8;
        let closest_match = images.iter().min_by_key(|image| image.color_distance((r, g, b))).unwrap();
        println!("color {}: {:?}", i, color);
        println!("closest color match: {:?} ({:?})", closest_match.path(), closest_match.color());

        let closest = ImageReader::open(closest_match.path()).unwrap().decode().unwrap();
        let closest = closest.resize_exact(OUTPUT_CELL_SIZE, OUTPUT_CELL_SIZE, image::imageops::FilterType::Triangle);

        let (x, y) = cells.index_to_coordinate(i);
        let x = x * OUTPUT_CELL_SIZE;
        let y = y * OUTPUT_CELL_SIZE;
        image::imageops::replace(&mut image, &closest, x as i64, y as i64);
        println!("index = {}, placed image at pixel ({}, {})\n", i, x, y);
    }

    image.into()
}