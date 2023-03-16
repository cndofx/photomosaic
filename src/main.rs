use image::{io::Reader as ImageReader, GenericImageView, Rgb};

const CELL_SIZE: u32 = 50;

fn main() {
    let mut img = ImageReader::open("img/input.png")
        .unwrap()
        .decode()
        .unwrap();

    let image_width = img.width();
    let image_height = img.height();
    dbg!(image_width, image_height);

    // crop image dimensions to multiples of CELL_SIZE
    let cropped_width = image_width - (image_width % CELL_SIZE);
    let cropped_height = image_height - (image_height % CELL_SIZE);
    dbg!(cropped_width, cropped_height);

    let new_image = img.crop(0, 0, cropped_width, cropped_height);
    new_image.save("img/output.png").unwrap();

    // get number of cells in the image
    let cells_x_count = cropped_width / CELL_SIZE;
    let cells_y_count = cropped_height / CELL_SIZE;
    let cells_count = cells_x_count * cells_y_count;
    dbg!(cells_x_count, cells_y_count, cells_count);

    // sum RGB values of all pixels in each cell
    let mut cell_colors: Vec<(u64, u64, u64)> = vec![(0, 0, 0); cells_count as usize];

    for (x, y, color) in new_image.pixels() {
        // get index into cell_colors array
        let cell_x = x / CELL_SIZE;
        let cell_y = y / CELL_SIZE;
        let i = (cell_x + cell_y * cells_x_count) as usize;

        // R
        cell_colors[i].0 += color.0[0] as u64;
        // G
        cell_colors[i].1 += color.0[1] as u64;
        // B
        cell_colors[i].2 += color.0[2] as u64;
    }

    // divide to get the average color
    let pixels_per_cell = (CELL_SIZE * CELL_SIZE) as u64;
    for cell in cell_colors.iter_mut() {
        cell.0 /= pixels_per_cell;
        cell.1 /= pixels_per_cell;
        cell.2 /= pixels_per_cell;
    }

    dbg!(cell_colors);
}
