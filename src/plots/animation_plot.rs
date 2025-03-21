use crate::{
    helper::file::get_current_dir,
    helper::file::save_image,
};
use std::fmt;
use std::{
    process::Command,
    fs,
    path::Path,
};

#[derive(Clone)]
pub enum EncodingSpeed {
    Ultrafast,
    Superfast,
    Veryfast,
    Faster,
    Fast,
    Medium,
    Slow,
    Slower,
    Veryslow,
}

impl fmt::Display for EncodingSpeed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let preset_str = match self {
            EncodingSpeed::Ultrafast => "ultrafast",
            EncodingSpeed::Superfast => "superfast",
            EncodingSpeed::Veryfast => "veryfast",
            EncodingSpeed::Faster => "faster",
            EncodingSpeed::Fast => "fast",
            EncodingSpeed::Medium => "medium",
            EncodingSpeed::Slow => "slow",
            EncodingSpeed::Slower => "slower",
            EncodingSpeed::Veryslow => "veryslow",
        };
        write!(f, "{}", preset_str)
    }
}

#[derive(Clone)]
pub struct AnimationPlotBuilder<'a> {
    ani: &'a Vec<Vec<Vec<(u8, u8, u8)>>>,
    path: Option<String>,
    framerate: Option<u32>,
    compression: Option<u32>,
    encoding_speed: Option<EncodingSpeed>,
    overwrite: Option<bool>,
}

/// Internal struct representing built values.
pub(crate) struct AnimationPlot<'a> {
    ani: &'a Vec<Vec<Vec<(u8, u8, u8)>>>,
    path: String,
    framerate: u32,
    compression: u32,
    encoding_speed: EncodingSpeed,
    overwrite: bool,
    temp_dir: String,
}

impl<'a> AnimationPlotBuilder<'a> {
    /// Create an array plot from a table of data.
    fn from(ani: &Vec<Vec<Vec<(u8, u8, u8)>>>) -> AnimationPlotBuilder {
        AnimationPlotBuilder {
            ani,
            path: None,
            framerate: None,
            compression: None,
            encoding_speed: None,
            overwrite: None,
        }
    }

    pub fn set_rel_path(&mut self, path: String) -> &mut Self {
        if path.contains(".mp4") {
            self.path = Some(get_current_dir() + &path);
        } else {
            self.path = Some(get_current_dir() + &path + &".mp4");
        }
        self
    }

    pub fn set_abs_path(&mut self, path: String) -> &mut Self {
        if path.contains(".mp4") {
            self.path = Some(path);
        } else {
            self.path = Some(path + &".mp4");
        }
        self
    }

    pub fn set_framerate(&mut self, framerate: u32) -> &mut Self {
        self.framerate = Some(framerate);
        self
    }

    pub fn set_compression(&mut self, compression: u32) -> &mut Self {
        self.compression = Some(compression);
        self
    }

    pub fn set_encoding_speed(&mut self, speed: EncodingSpeed) -> &mut Self {
        self.encoding_speed = Some(speed);
        self
    }

    pub fn set_overwrite(&mut self, do_overwrite: bool) -> &mut Self {
        self.overwrite = Some(do_overwrite);
        self
    }

    fn build(&mut self) -> AnimationPlot {
        AnimationPlot {
            ani: self.ani,
            path: self.path.clone().unwrap_or_else(|| get_current_dir() + &"output.mp4"),
            framerate: self.framerate.unwrap_or(30),
            compression: self.compression.unwrap_or(23),
            encoding_speed: self.encoding_speed.clone().unwrap_or(EncodingSpeed::Fast),
            overwrite: self.overwrite.unwrap_or(false),
            temp_dir: get_current_dir() + "temp_dir_for_ffmpeg/",
        }
    }

    pub fn save(&mut self) {
        self.build().save();
    }

    /// Instead of saving an animation from a Vec of images, it allows
    /// using the builder parameters to save arbitrary image files.
    /// Given a function which moves images to a temporary directory,
    /// it will create an animation from it.
    /// Tempdir DOES NOT end with a '/'
    /// Images should be named 1.png, 2.png, etc.
    pub fn save_arbitrary_images(&mut self, image_mover: impl Fn(&str)) {
        self.build().save_arbitrary_images(image_mover);
    }
}

impl<'a> AnimationPlot<'a> {
    fn create_temp_dir(&self) {
        let dir_path = Path::new(&self.temp_dir);
        fs::create_dir_all(dir_path).expect("Could not create temporary directory.");
    }

    fn save_images(&self) {
        self.ani
        .iter()
        .enumerate()
        .for_each(|(i, img)|
        save_image(&img, &(self.temp_dir.clone() + &i.to_string() + ".png"))
        );
    }

    fn run_ffmpeg_commands(&self) {
        let input_path = self.temp_dir.clone() + "%d.png";

        let status = Command::new("ffmpeg")
            .arg("-framerate")
            .arg(self.framerate.to_string())
            .arg("-i")
            .arg(input_path) 
            .arg("-vf")
            .arg("scale=ceil(iw/2)*2:ceil(ih/2)*2")  // crops dimensions to multiple of 2
            .arg("-vcodec")
            .arg("libx264") // .mp4
            .arg("-crf")
            .arg(self.compression.to_string())
            .arg("-pix_fmt")
            .arg("yuv420p") // Ensures compatibility
            .arg("-preset")
            .arg(self.encoding_speed.to_string())
            .arg(if self.overwrite {"-y"} else {"-n"})
            .arg(self.path.clone())
            .status()
            .expect("Failed to execute FFmpeg command");

        if !status.success() {
            panic!("Failed to execute FFmpeg command");
        }
    }

    fn delete_temporary_dir(&self) {
        let dir_path = Path::new(&self.temp_dir);
        if dir_path.exists() {
            fs::remove_dir_all(dir_path)
                .expect("Failed to delete temp directory and its contents");
        }
    }

    pub fn save(&self) {
        self.create_temp_dir();
        self.save_images();
        self.run_ffmpeg_commands();
        self.delete_temporary_dir();
    }

    pub fn save_arbitrary_images(&self, image_mover: impl Fn(&str)) {
        self.create_temp_dir();
        image_mover(&self.temp_dir);
        self.run_ffmpeg_commands();
        self.delete_temporary_dir();
    }
}

pub fn animation_plot<'a>(ani: &'a Vec<Vec<Vec<(u8, u8, u8)>>>) -> AnimationPlotBuilder<'a> {
    AnimationPlotBuilder::from(ani)
}