use image::RgbImage;
use crate::{image as img_utils, font, ascii};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CharsetPreset {
    #[default]
    Default,
    Dense,
    Blocks,
}

impl CharsetPreset {
    pub fn chars(&self) -> &'static str {
        match self {
            CharsetPreset::Default => "@%#*+=-:. ",
            CharsetPreset::Dense => "@M#W$9876543210?!abc;:+=-,._ ",
            CharsetPreset::Blocks => "█▓▒░ ",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AsciiOptions {
    pub width: u32,
    pub aspect: f32,
    pub invert: bool,
    pub color: bool,
    pub charset: Option<String>,
    pub charset_preset: CharsetPreset,
}

impl Default for AsciiOptions {
    fn default() -> Self {
        Self {
            width: 100,
            aspect: 0.5,
            invert: false,
            color: false,
            charset: None,
            charset_preset: CharsetPreset::Default,
        }
    }
}

impl AsciiOptions {
    pub fn resolved_charset(&self) -> String {
        if let Some(ref s) = self.charset {
            return s.clone();
        }
        self.charset_preset.chars().to_string()
    }
}

pub fn image_to_ascii(img: &RgbImage, options: &AsciiOptions) -> String {
    let (block_w, block_h) = img_utils::compute_block_size(
        img.width(),
        img.height(),
        options.width,
        options.aspect,
    );

    let blocks = img_utils::sample_image_blocks(
        img,
        block_w,
        block_h,
        img.width(),
        img.height(),
        options.invert,
    );

    let charset = options.resolved_charset();
    let char_bitmap = font::build_charset(&charset);
    ascii::render(blocks, &char_bitmap, options.color)
}
