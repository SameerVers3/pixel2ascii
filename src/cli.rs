use std::path::PathBuf;

use clap::{ArgGroup, Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "pixel2ascii", about = "Convert images to ASCII art", version)]
#[command(author, version = env!("CARGO_PKG_VERSION"))]
#[command(group(ArgGroup::new("charset_choice").args(["charset", "charset_preset"])))]
pub struct Cli {
    /// Path to input image file
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    // Rendering
    /// Maximum ASCII character width (must be > 0)
    #[arg(short = 'w', long = "width", default_value_t = 100)]
    pub width: u32,

    /// Character height / width ratio. Ignored when --no-aspect is set.
    #[arg(long = "aspect", default_value_t = 0.5_f32)]
    pub aspect: f32,

    /// Disable aspect correction (block height = block width)
    #[arg(long = "no-aspect")]
    pub no_aspect: bool,

    /// Invert luminance mapping
    #[arg(long = "invert")]
    pub invert: bool,

    // Color
    /// Enable ANSI truecolor foreground
    #[arg(short = 'c', long = "color")]
    pub color: bool,

    /// Use background color mode instead of foreground (requires --color)
    #[arg(long = "bg")]
    pub bg: bool,

    /// Force grayscale output (overrides --color)
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// Custom character set ordered from dark -> light. Must be length >= 2.
    #[arg(long = "charset")]
    pub charset: Option<String>,

    /// Charset preset to use (mutually exclusive with --charset)
    #[arg(long = "charset-preset", value_enum, default_value_t = CharsetPreset::Default)]
    pub charset_preset: CharsetPreset,

    /// Write output to file instead of stdout
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Disable non-error logs
    #[arg(long = "quiet")]
    pub quiet: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum CharsetPreset {
    Default,
    Dense,
    Blocks,
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        if !self.input.exists() {
            return Err(format!("input path '{}' does not exist", self.input.display()));
        }
        if !self.input.is_file() {
            return Err(format!("input path '{}' is not a file", self.input.display()));
        }

        if self.width == 0 {
            return Err("--width must be > 0".to_string());
        }
        if self.width > 5000 {
            return Err("--width is unreasonably large; maximum is 5000".to_string());
        }

        if self.aspect <= 0.0 {
            return Err("--aspect must be > 0".to_string());
        }

        if self.bg && !self.color {
            return Err("--bg requires --color to be set".to_string());
        }


        // charset validation: if provided, must be >= 2 characters and not all whitespace
        if let Some(ref s) = self.charset {
            let trimmed = s.trim();
            if trimmed.len() < 2 {
                return Err("--charset must contain at least 2 non-whitespace characters".to_string());
            }
        }

        Ok(())
    }

    pub fn resolved_charset(&self) -> Vec<char> {
        if let Some(ref s) = self.charset {
            return s.chars().collect();
        }

        match self.charset_preset {
            CharsetPreset::Default => "@%#*+=-:. ".chars().collect(),
            CharsetPreset::Dense => "@M#W$9876543210?!abc;:+=-,._ ".chars().collect(),
            CharsetPreset::Blocks => "█▓▒░ ".chars().collect(),
        }
    }
}