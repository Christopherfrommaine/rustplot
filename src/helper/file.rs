use std::{env::current_dir, fs::write};
use log::warn;
use image::{ImageBuffer, Rgb};

pub fn get_current_dir() -> String {
    match current_dir() {
        Ok(path) => {
            let o = path.to_str().unwrap().to_string() + "/";
            println!("get_current_dir output: {o}");
            return o;
        }
        Err(_e) => {
            warn!("Failed to get working directory. Using ~/cgrustplot/");
            return "~/cgrustplot/".to_string();
        }
    }
}

pub fn save_to_file(s: &str, path: &str) {
    let r = write(path, s);
    if let Err(e) = r {warn!("Failed to write to '{path}': {e}");}
}

pub fn save_image(img: &Vec<Vec<(u8, u8, u8)>>, path: &str) {
    let height = img.len() as u32;
    let width = if height > 0 {img[0].len() as u32} else {0};

    let buffer: Vec<u8> = img
        .iter()
        .flat_map(|row|
            row
            .iter()
            .flat_map(|(r, g, b)| vec![*r, *g, *b])
        ).collect();

    let image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, buffer).expect("Buffer size mismatch");

    let r = image.save(path);
    if let Err(e) = r {warn!("Failed to save image to '{path}': {e}");}
}