use std::{fs::read_dir, path::PathBuf};

use clap::Parser;
use image::{io::Reader as ImageReader, DynamicImage};

#[derive(Parser)]
struct Cli {
    /// Directory to source images from
    source_directory: PathBuf,
    /// Directory to save output images to (defaults to the source directory)
    // #[arg(default_value_os_t = PathBuf::from("."))]
    output_directory: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let output_directory = cli.output_directory.unwrap_or_else(|| cli.source_directory.clone());
    std::fs::create_dir_all(&output_directory).unwrap();

    // get paths to all files in the source directory
    let mut paths: Vec<PathBuf> = Vec::new();
    let files_iter = read_dir(&cli.source_directory).unwrap();
    for file in files_iter {
        match file {
            Ok(file) => {
                if file.file_type().unwrap().is_file() {
                    paths.push(file.path());
                }
            }
            Err(e) => {
                eprintln!("Unable to access DirEntry: {}", e);
            }
        }
    }

    // crop all files in the list
    for path in paths.iter() {
        if path.file_name().unwrap().to_str().unwrap().contains("_cropped") {
            eprintln!("{:?} is already cropped, skipping", path.file_name().unwrap());
            continue;
        }

        // get path to write new image to
        let filename = path.file_stem().unwrap();
        let mut new_filename = filename.to_owned();
        new_filename.push("_cropped");

        let mut out_path = output_directory.clone();
        out_path.push(new_filename);
        out_path.set_extension("png");

        match ImageReader::open(&path).unwrap().decode() {
            Ok(mut image) => {
                let new_image = square_image(&mut image);
                new_image.save(out_path).unwrap();
            }
            Err(e) => {
                eprintln!("Skipping {:?} due to error: {}", &path, e);
            }
        }
    }
}

fn square_image(image: &mut DynamicImage) -> DynamicImage {
    let width = image.width();
    let height = image.height();
    let smallest = std::cmp::min(width, height);
    image.crop(0, 0, smallest, smallest)
}