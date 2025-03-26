//! Helper file for working with and saving to files.
//! 
//! Most functions will not panic if they fail, and will call warn!()

use std::{env::current_dir, fs::write};
use log::warn;
use image::{ImageBuffer, Rgb};

/// Returns the current directory of the project.
/// 
/// If it fails, it will warn!() and return "~/cgrustplot/"
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::file::get_current_dir;
/// let result = get_current_dir();  // For me, "/home/chris/Programming/Rust/rustplot"
/// assert!(result.contains("/"));
/// ```
pub fn get_current_dir() -> String {
    match current_dir() {
        Ok(path) => {
            let o = path.to_str().unwrap().to_string() + "/";
            return o;
        }
        Err(_e) => {
            warn!("Failed to get working directory. Using ~/cgrustplot/");
            return "~/cgrustplot/".to_string();
        }
    }
}

/// Saves a string to a file.
/// 
/// If it fails, it will warn!().
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::file::{save_to_file, get_current_dir};
/// save_to_file("test", &(get_current_dir() + "testoutput/doctest_save_to_file.txt"));
/// // testoutput/doctest_save_to_file.txt now contains "test"
/// ```
pub fn save_to_file(s: &str, path: &str) {
    let r = write(path, s);
    if let Err(e) = r {warn!("Failed to write to '{path}': {e}");}
}

/// Saves an image (represented by a table of RGB values) to an image file.
/// 
/// Uses image craate, and optimized to use imagebuffer instead of put_pixel.
/// 
/// Also paralellized with rayon for conversion from table of RGB to
/// flattened pixel buffer.
/// 
/// If it fails, it will warn!().
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::file::{save_image, get_current_dir};
/// save_image(&vec![vec![(255, 255, 255), (0, 0, 0)], vec![(0, 0, 0), (255, 255, 255)]], &(get_current_dir() + "testoutput/doctest_save_image.png"));
/// // testoutput/doctest_save_image.png now contains:
/// // █░
/// // ░█
/// ```
/// 
/// # Notes
/// 
/// Internally copies and reformats the image, so may need optimization in the future.
/// 
pub fn save_image(img: &Vec<Vec<(u8, u8, u8)>>, path: &str) {
    use rayon::prelude::*;
    
    let height = img.len() as u32;
    let width = if height > 0 {img[0].len() as u32} else {0};

    let buffer: Vec<u8> = img
        .par_iter()
        .flat_map_iter(|row|
            row
            .iter()
            .flat_map(|(r, g, b)| [*r, *g, *b])
        ).collect();

    let image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, buffer).expect("Buffer size mismatch");

    let r = image.save(path);
    if let Err(e) = r {warn!("Failed to save image to '{path}': {e}");}
}