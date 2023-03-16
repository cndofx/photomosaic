use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ImageRecord {
    path: PathBuf,
    average_color: (u8, u8, u8),
}

impl ImageRecord {
    pub fn new(path: impl Into<PathBuf>, average_color: (u8, u8, u8)) -> Self {
        ImageRecord {
            path: path.into(),
            average_color,
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        self.average_color
    }

    pub fn color_distance(&self, other: (u8, u8, u8)) -> f32 {
        let r1 = self.average_color.0 as f32;
        let g1 = self.average_color.1 as f32;
        let b1 = self.average_color.2 as f32;
        let r2 = other.0 as f32;
        let g2 = other.1 as f32;
        let b2 = other.2 as f32;
        ((r1 - r2) + (g1 - g2) + (b1 - b2)).sqrt()
    }
}
