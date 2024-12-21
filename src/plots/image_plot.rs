use crate::plots::array_plot::array_plot;
use crate::helper::image::*;

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