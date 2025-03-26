use std::{hash::{Hash, Hasher}, process::Command};
use log::warn;
use crate::helper::file::{save_to_file, get_current_dir};

/// Builder struct for rendering text to an image.
/// 
/// `magick` must be installed.
/// 
/// # Notes
/// 
/// `font_color` and `background_color` must be an acceptable color definition for magick
pub struct RenderableTextBuilder<'a> {
    s: String,
    font_path: Option<&'a str>,
    font_color: Option<&'a str>,
    font_size: Option<u32>,
    background_color: Option<&'a str>,
}

struct RenderableText<'a> {
    s: String,
    font_path: &'a str,
    font_color: &'a str,
    font_size: u32,
    background_color: &'a str,
}

impl<'a> RenderableTextBuilder<'a> {
    pub fn from(s: String) -> Self {
        RenderableTextBuilder {
            s,
            font_path: None,
            font_color: None,
            font_size: None,
            background_color: None,
        }
    }

    pub fn set_font_path<'b: 'a>(&mut self, font_path: &'b str) -> &Self {
        self.font_path = Some(font_path);
        self
    }

    pub fn set_font_color<'b: 'a>(&mut self, font_color: &'b str) -> &Self {
        self.font_color = Some(font_color);
        self
    }

    pub fn set_font_size(&mut self, font_size: u32) -> &Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn set_background_color<'b: 'a>(&mut self, color: &'b str) -> &Self {
        self.background_color = Some(color);
        self
    }

    fn build(&self) -> RenderableText<'a> {
        RenderableText {
            s: self.s.clone(),
            font_path: self.font_path.unwrap_or("DejaVu-Sans-Mono"),  // "works on my machine". TODO: possibly use magick commands to search for existing fonts, grep for "mono" and then use the first?
            font_color: self.font_color.unwrap_or("white"),
            font_size: self.font_size.unwrap_or(24),
            background_color: self.background_color.unwrap_or("black"),
        }
    }

    /// Saves an image to a file
    pub fn save(&self, path: &str) {
        self.build().save(path);
    }
}

impl<'a> RenderableText<'a> {
    fn save(&self, path: &str) {
        let mut hasher = std::hash::DefaultHasher::new();
        path.hash(&mut hasher);

        let temp_text_path = get_current_dir() + "temp_file_for_image_rendering_" + &((hasher.finish() & 0xFFFF_FFFF) as u32).to_string() + ".txt";
        save_to_file(&self.s, &temp_text_path);

        let mut binding = Command::new("magick");
        let cmd = binding
            .arg("-font")
            .arg(self.font_path)
            .arg("-fill")
            .arg(self.font_color)
            .arg("-pointsize")
            .arg(self.font_size.to_string())
            .arg("-background")
            .arg(self.background_color)
            .arg(format!("label:@{temp_text_path}"))
            .arg(path);

        if let Err(e) = cmd.status() {warn!("{e}");}
        
        if let Err(e) = std::fs::remove_file(temp_text_path) {warn!("Could not remove temporary file during the creation of {path}: {e}")}
    }
}