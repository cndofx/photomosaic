use image::{io::Reader as ImageReader, Rgb, RgbImage};
use rayon::prelude::*;

use photomosaic::{average_image_color, cells::Cells, get_file_paths, image_record::ImageRecord};

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
    pixelated_image.save("img/output.png").unwrap();

    // ========================

    let files = get_file_paths("img/sources/cropped/");
    // files.into_par_iter().for_each(|file| {
    //     let img = ImageReader::open(&file).unwrap().decode().unwrap();
    //     let color = average_image_color(&img);
    //     println!(
    //         "file: {:?}\naverage image color: {:?}\n",
    //         &file.file_name().unwrap(),
    //         color
    //     );
    // });

    let records: Vec<ImageRecord> = files.into_par_iter().map(|path| {
        let img = ImageReader::open(&path).unwrap().decode().unwrap();
        let color = average_image_color(&img);
        ImageRecord::new(path, color)
    }).collect();

    dbg!(&records);

    for (i, color) in cells.iter().enumerate() {
        println!("color {}: {:?}", i, color);
        let r = color.0 as u8;
        let b = color.1 as u8;
        let g = color.2 as u8;
        let closest_match = records.iter().min_by_key(|record| record.color_distance((r, g, b))).unwrap();
        println!("closest color match: {:?}\n", closest_match.path());
    }
}
