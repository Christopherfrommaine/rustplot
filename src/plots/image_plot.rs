use crate::plots::array_plot::array_plot;
use crate::helper::image::*;
use crate::helper::mat_plot_lib::pyplot;

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

    pub fn set_rel_path(&mut self, path: String) -> &mut Self {
        if path.contains(".png") {
            self.path = Some(get_current_dir() + &path);
        } else {
            self.path = Some(get_current_dir() + &path + &".png");
        }
        self
    }

    pub fn set_abs_path(&mut self, path: String) -> &mut Self {
        if path.contains(".png") {
            self.path = Some(path);
        } else {
            self.path = Some(path + &".png");
        }
        self
    }

    fn build(&mut self) -> ImagePlot {
        ImagePlot {
            img: self.img,
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

    pub fn pyplot(&mut self) {
        self.build().pyplot(None);
    }

    pub fn save_pyplot(&mut self, path: &str) {
        self.build().pyplot(Some(path));
    }

    pub fn save(&mut self) {
        self.build().save();
    }
}

impl<'a> ImagePlot<'a> {
    fn plot(&self) -> String {
        let brightnesses: Vec<Vec<u32>> = self.img.iter().map(|row| row.iter().map(|p| p.0 as u32 + p.1 as u32 + p.2 as u32).collect()).collect();
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
        let command = format!("plt.imshow(np.array({:?}))", self.img);
        pyplot(&command, None, None, None, path);
    }

    fn save(&self) {
        save_image_to_path(&self.img, self.path.clone());
    }
}

pub fn image_plot<'a>(img: &'a Vec<Vec<(u8, u8, u8)>>) -> ImagePlotBuilder<'a> {
    ImagePlotBuilder::from(img)
}

pub fn convert_from_hsv(hsv: &Vec<Vec<(u8, u8, u8)>>) -> Vec<Vec<(u8, u8, u8)>> {
    hsv.iter().map(|row| row.into_iter().map(|pixel| hsv_to_rgb(*pixel)).collect()).collect()
}