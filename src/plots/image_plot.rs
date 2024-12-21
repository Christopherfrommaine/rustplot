use image::{Rgb, RgbImage};
use std::env::current_dir;
use crate::plots::array_plot::array_plot;

fn save_image_to_path(img: &Vec<Vec<(u8, u8, u8)>>, path: String) {
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

fn get_current_dir() -> String {
    match current_dir() {
        Ok(path) => {
            return path.to_str().unwrap().to_string();
        }
        Err(_e) => {
            eprintln!("Failed to get working directory. Using home directory.");
            return "~/".to_string();
        }
    }
}

fn hsv_to_rgb(hsv: (u8, u8, u8)) -> (u8, u8, u8) {
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

pub struct ImagePlotBuilder {
    img: Vec<Vec<(u8, u8, u8)>>,
    path: Option<String>,
}

/// Internal struct representing built values.
pub(crate) struct ImagePlot {
    img: Vec<Vec<(u8, u8, u8)>>,
    path: String,
}

impl ImagePlotBuilder {
    /// Create an array plot from a table of data.
    fn from(img: Vec<Vec<(u8, u8, u8)>>) -> ImagePlotBuilder {
        ImagePlotBuilder {
            img,
            path: None
        }
    }

    pub fn set_rel_path(&mut self, path: String) -> &mut ImagePlotBuilder {
        if path.contains(".png") {
            self.path = Some(get_current_dir() + &path);
        } else {
            self.path = Some(get_current_dir() + &path + &".png");
        }
        
        self
    }

    pub fn set_abs_path(&mut self, path: String) -> &mut ImagePlotBuilder {
        if path.contains(".png") {
            self.path = Some(path);
        } else {
            self.path = Some(path + &".png");
        }
        self
    }

    pub fn convert_from_hsv(&mut self) -> &mut ImagePlotBuilder {
        self.img = self.img.iter().map(|row| row.into_iter().map(|pixel| hsv_to_rgb(*pixel)).collect()).collect();
        self
    }

    fn build(&mut self) -> ImagePlot {
        ImagePlot {
            img: self.img.clone(),
            path: self.path.clone().unwrap_or(get_current_dir() + &"output.png"),
        }
    }

    /// Returns the plotted data as a string
    pub fn as_string(&mut self) -> String {
        self.build().as_string()
    }

    /// Displays the plotted data with println
    pub fn print(&mut self) {
        self.build().print();
    }

    pub fn save(&mut self) {
        self.build().save();
    }
}

impl ImagePlot {
    fn plot(&self) -> String {
        let brightnesses: Vec<Vec<u32>> = self.img.iter().map(|row| row.iter().map(|p| p.0 as u32 + p.1 as u32 + p.2 as u32).collect()).collect();
        array_plot(&brightnesses)
        .set_axes(false)
        .set_title(self.path.clone())
        .as_string()
    }

    pub fn as_string(&self) -> String {
        self.plot()
    }

    pub fn print(&self) {
        println!("{}", self.as_string());
    }

    pub fn save(&self) {
        save_image_to_path(&self.img, self.path.clone());
    }
}

pub fn image_plot(img: Vec<Vec<(u8, u8, u8)>>) -> ImagePlotBuilder {
    ImagePlotBuilder::from(img)
}