//! # Animation Plot
//! Creates a video or animation from a series of frames or existing image files.
//! 
//! # Example
//! ```
//! use cgrustplot::plots::animation_plot::*;
//! let frames: Vec<Vec<Vec<(u8, u8, u8)>>> = (0..100).map(|frame| (0..50).map(|y| (0..100).map(|x| if frame == x {(255, 255, 255)} else {(0, 0, 0)}).collect()).collect()).collect();
//! animation_plot(&frames).set_rel_path("testoutput/doctest_animation_plot.mp4").save();
//! ```

use crate::{
    helper::file::get_current_dir,
    helper::file::save_image,
};
use std::fmt;
use log::warn;
use std::{
    process::Command,
    fs,
    path::Path,
};

/// Represents possible values for encoding speed for ffmpeg.
#[derive(Debug, Clone)]
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

/// Builder for an Animation Plot
/// Set various options for rendering the output.
/// 
/// # Options
/// 
/// * `ani` - The inputted vector of frames.
/// * `path` - The path to save the output video file to. Default is "./output.mp4".
/// * `framerate` - The framerate of the output video.
/// * `compression` - The CRF value for FFmpeg: higher is more compressed. Default is 23.
/// * `encoding_speed` - The encoding speed for FFmpeg, given by `animation_plot::EncodingSpeed` enum. Default is Fast.
/// * `overwrite` - Whether or not to overwrite an existing file of the same name. Default is false.
/// 
/// # Notes
/// 
/// FFmpeg must be installed.
/// 
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

    pub fn set_rel_path(&mut self, path: &str) -> &mut Self {
        if path.contains(".mp4") {
            self.path = Some(get_current_dir() + path);
        } else {
            self.path = Some(get_current_dir() + path + ".mp4");
        }
        self
    }

    pub fn set_abs_path(&mut self, path: &str) -> &mut Self {
        if path.contains(".mp4") {
            self.path = Some(path.to_string());
        } else {
            self.path = Some(path.to_string() + ".mp4");
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
        use rayon::prelude::*;

        self.ani
        .par_iter()
        .enumerate()
        .for_each(|(i, img)|
        save_image(&img, &(self.temp_dir.clone() + &i.to_string() + ".png"))
        );
    }

    fn run_ffmpeg_commands(&self) {
        let input_path = self.temp_dir.clone() + "%d.png";

        let result = Command::new("ffmpeg")
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
            .status();

        if let Ok(status) = result {
            if !status.success() {
                warn!("Failed to execute FFmpeg command: {status}");
            }
        } else {
            warn!("Error status for FFmpeg command: {result:?}");
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

/// Creates a video or animation from a series of frames or existing image files. 
/// 
/// # Options
/// 
/// * `ani` - The inputted vector of frames.
/// * `path` - The path to save the output video file to. Default is "./output.mp4".
/// * `framerate` - The framerate of the output video.
/// * `compression` - The CRF value for FFmpeg: higher is more compressed. Default is 23.
/// * `encoding_speed` - The encoding speed for FFmpeg, given by `animation_plot::EncodingSpeed` enum. Default is Fast.
/// * `overwrite` - Whether or not to overwrite an existing file of the same name. Default is false.
/// 
/// # Notes
/// 
/// FFmpeg must be installed.
/// 
pub fn animation_plot<'a>(ani: &'a Vec<Vec<Vec<(u8, u8, u8)>>>) -> AnimationPlotBuilder<'a> {
    AnimationPlotBuilder::from(ani)
}