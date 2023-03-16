use image::{DynamicImage, GenericImageView};
use rayon::prelude::IntoParallelRefIterator;

use crate::crop_image;

pub const CELL_SIZE: u32 = 5;

#[derive(Debug, Clone)]
pub struct Cells {
    cells: Vec<(u64, u64, u64)>,
    count_x: u32,
    count_y: u32,
}

impl Cells {
    /// Creates a list of cells containing the average colors
    /// of the corresponding pixels in the given image
    pub fn from_image(mut image: &mut DynamicImage) -> Self {
        let image = crop_image(&mut image);
        let image_width = image.width();
        let image_height = image.height();

        let count_x = image_width / CELL_SIZE;
        let count_y = image_height / CELL_SIZE;
        let count = count_x * count_y;
        let mut cells = vec![(0, 0, 0); count as usize];

        for (x, y, color) in image.pixels() {
            let r = color.0[0] as u64;
            let g = color.0[1] as u64;
            let b = color.0[2] as u64;
            let cell_x = x / CELL_SIZE;
            let cell_y = y / CELL_SIZE;
            let i = (cell_x + cell_y * count_x) as usize;
            cells[i].0 += r;
            cells[i].1 += g;
            cells[i].2 += b;
        }

        let pixels_per_cell = (CELL_SIZE * CELL_SIZE) as u64;
        for cell in cells.iter_mut() {
            cell.0 /= pixels_per_cell;
            cell.1 /= pixels_per_cell;
            cell.2 /= pixels_per_cell;
        }

        Cells {
            cells,
            count_x,
            count_y,
        }
    }

    /// The width of the cell array
    pub fn width(&self) -> u32 {
        self.count_x
    }

    /// The height of the cell array
    pub fn height(&self) -> u32 {
        self.count_y
    }

    /// The total number of cells in the array
    pub fn count(&self) -> u32 {
        self.count_x * self.count_y
    }

    pub fn index_to_coordinate(&self, index: usize) -> (u32, u32) {
        (index as u32 % self.count_x, index as u32 / self.count_x)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (u64, u64, u64)> {
        self.cells.iter()
    }

    pub fn par_iter(&self) -> rayon::slice::Iter<'_, (u64, u64, u64)> {
        self.cells.par_iter()
    }
}