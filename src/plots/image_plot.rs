//! # Image Plot
//! Creates an image from a table of (R, G, B) values.
//! 
//! # Functions
//! 
//! * `image_plot` - Generates an ImagePlotBuilder from a table of RGB.
//! * `convert_from_hsv` - Converts an HSV table to an RGB table.
//! 


use crate::{
    helper::{
        file::{get_current_dir, save_image, save_to_file},
        mat_plot_lib::pyplot,
        rendering::RenderableTextBuilder,
    },
    plots::array_plot::array_plot,
};
use rayon::prelude::*;

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

/// Builder for an Image Plot
/// Set various options for the image.
/// 
/// # Options
/// 
/// * `img` - Input table of (R, G, B) representing the image.
/// * `path` - Path to save the image to. Default is "./output.png".
/// 
#[derive(Clone)]
pub struct ImagePlotBuilder<'a> {
    img: &'a Vec<Vec<(u8, u8, u8)>>,
    path: Option<String>,
}

/// Internal struct representing built values.
struct ImagePlot<'a> {
    img: &'a Vec<Vec<(u8, u8, u8)>>,
    path: String,
}

impl<'a> ImagePlotBuilder<'a> {
    /// Create an array plot from a table of data.
    fn from<'b: 'a>(img: &'b Vec<Vec<(u8, u8, u8)>>) -> Self {
        ImagePlotBuilder {
            img,
            path: None
        }
    }

    pub fn set_rel_path(&mut self, path: &str) -> &mut Self {
        if path.contains(".") {
            self.path = Some(get_current_dir() + path);
        } else {
            self.path = Some(get_current_dir() + path + ".png");
        }
        self
    }

    pub fn set_abs_path(&mut self, path: &str) -> &mut Self {
        if path.contains(".") {
            self.path = Some(path.to_string());
        } else {
            self.path = Some(path.to_string() + ".png");
        }
        self
    }

    fn build(&self) -> ImagePlot {
        ImagePlot {
            img: self.img,
            path: self.path.clone().unwrap_or_else(|| get_current_dir() + &"output.png"),
        }
    }

    /// Returns a monochrome text render as a string
    pub fn as_string(&self) -> String {
        self.build().as_string()
    }

    /// Displays a monochrome text render with println
    pub fn print(&self) {
        self.build().print();
    }

    /// Saves a monochrome text render to a file
    pub fn save_as_text(&self, path: &str) {
        save_to_file(&self.build().as_string(), path);
    }

    /// Saves the image to a file.
    pub fn save(&self) {
        self.build().save();
    }

    /// Returns a rendered text builder to render a string
    pub fn as_image(&self) -> RenderableTextBuilder {
        RenderableTextBuilder::from(self.build().as_string())
    }

    /// Displays the plot's data using pyplot
    pub fn pyplot(&self) {
        self.build().pyplot(None);
    }

    /// Saves the plot's data using pyplot
    pub fn save_pyplot(&self, path: &str) {
        self.build().pyplot(Some(path));
    }

    /// Returns the unformatted text content of a plot
    #[allow(dead_code)]
    pub(crate) fn plot(&self) -> String {
        self.build().plot()
    }
}

impl<'a> ImagePlot<'a> {
    fn plot(&self) -> String {
        let brightnesses: Vec<Vec<u32>> = self.img.par_iter().map(|row| row.iter().map(|p| p.0 as u32 + p.1 as u32 + p.2 as u32).collect()).collect();
        array_plot(&brightnesses)
        .set_axes(false)
        .set_title(&self.path)
        .as_string()
    }

    fn as_string(&self) -> String {
        self.plot()
    }

    fn print(&self) {
        println!("{}", self.as_string());
    }

    fn pyplot(&self, path: Option<&str>) {
        let command = format!("imshow(np.array({:?}))", self.img);
        pyplot(&command, None, None, None, path);
    }

    fn save(&self) {
        save_image(&self.img, &self.path);
    }
}

/// Creates an image from a table of (R, G, B) values.
/// 
/// # Example
/// 
/// ```
/// use cgrustplot::plots::image_plot::image_plot;
/// 
/// let image: Vec<Vec<(u8, u8, u8)>> = (0..1080).map(|r| (0..1920).map(|c| (0.01 * r as f64).sin() * (0.01 * c as f64).sin()).map(|x| (127. * (1. + x)) as u8).map(|x| (x, x, x)).collect()).collect();
/// image_plot(&image).set_rel_path("testoutput/doctest_image_plot.png").save();
/// 
/// ```
/// 
/// # Options
/// 
/// * `img` - Input table of (R, G, B) representing the image.
/// * `path` - Path to save the image to. Default is "./output.png".
/// 
pub fn image_plot<'a>(img: &'a Vec<Vec<(u8, u8, u8)>>) -> ImagePlotBuilder<'a> {
    ImagePlotBuilder::from(img)
}

/// Converts a HSV image (represented as a table of (H, S, V)) to an RGB image.
pub fn convert_from_hsv(hsv: &Vec<Vec<(u8, u8, u8)>>) -> Vec<Vec<(u8, u8, u8)>> {
    hsv.par_iter().map(|row| row.into_iter().map(|pixel| hsv_to_rgb(*pixel)).collect()).collect()
}