use std::process::Command;
use log::warn;

/// Builder for rendering text to an image
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
            font_path: self.font_path.unwrap_or("Courier"),
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
    pub fn save(&self, path: &str) {
        let r = Command::new("magick")
            .arg("-font")
            .arg(self.font_path)
            .arg("-fill")
            .arg(self.font_color)
            .arg("-pointsize")
            .arg(self.font_size.to_string())
            .arg("-background")
            .arg(self.background_color)
            .arg(format!("label:\"{}\"", self.s))
            .arg(path)
            .status();

        if let Err(e) = r {warn!("{e}");}
    }
}