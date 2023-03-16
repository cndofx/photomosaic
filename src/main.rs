use image::{io::Reader as ImageReader, GenericImageView, Rgb, RgbImage, DynamicImage};

use photomosaic::cells::{CELL_SIZE, Cells};

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
}

