use image::{io::Reader as ImageReader, GenericImageView};

const CELL_SIZE: u32 = 50;

fn main() {
    let mut img = ImageReader::open("img/input.png")
        .unwrap()
        .decode()
        .unwrap();

    let width = img.width();
    let height = img.height();
    dbg!(width, height);

    let new_width = width - (width % CELL_SIZE);
    let new_height = height - (height % CELL_SIZE);
    dbg!(new_width, new_height);

    let cells_x = new_width / CELL_SIZE;
    let cells_y = new_height / CELL_SIZE;
    dbg!(cells_x, cells_y);

    let new_image = img.crop(0, 0, new_width, new_height);
    new_image.save("img/output.png").unwrap();

    // for (x, y, color) in new_image.pixels() {
    //     println!("pixel belongs to cell at ({}, {})", x / CELL_SIZE, y / CELL_SIZE);
    // }

    // dbg!(&img);
}
