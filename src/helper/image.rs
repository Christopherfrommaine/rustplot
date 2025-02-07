use image::{Rgb, RgbImage};
use std::env::current_dir;

pub fn get_current_dir() -> String {
    match current_dir() {
        Ok(path) => {
            return path.to_str().unwrap().to_string() + "\\";
        }
        Err(_e) => {
            eprintln!("Failed to get working directory. Using home directory.");
            return "~/".to_string();
        }
    }
}

pub fn hsv_to_rgb(hsv: (u8, u8, u8)) -> (u8, u8, u8) {
    let (h, s, v) = hsv;

    let h = h as f64 * 360.0 / 255.0; // Scale hue to [0, 360)
    let s = s as f64 / 255.0;         // Scale saturation to [0, 1]
    let v = v as f64 / 255.0;         // Scale value to [0, 1]

    let c = v * s; // Chroma
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = match h as u16 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0), // Default to black if hue is out of range
    };

    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;
    (r, g, b)
}


// TODO: This is quite inefficient. Would another library be better?
// I don't want to have to set pixels individually
pub fn save_image_to_path(img: &Vec<Vec<(u8, u8, u8)>>, path: String) {
    // Get the image dimensions
    let width = img[0].len() as u32;
    let height = img.len() as u32;

    let mut o = RgbImage::new(width, height);

    img.iter().enumerate().for_each(|(y, row)|
        row.iter().enumerate().for_each(|(x, rgb)|
            o.put_pixel(x as u32, y as u32, Rgb([rgb.0, rgb.1, rgb.2]))
        )
    );

    o.save(path.clone()).expect(&format!("Failed to save image to path {path}"));
}