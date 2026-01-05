use font8x8::{BASIC_FONTS, BLOCK_FONTS, UnicodeFonts};

#[derive(Clone)]
pub struct CharBitmap {
    pub ch: char,
    pub bitmap: Vec<Vec<u8>>,
    pub intensity: f32,
}

pub fn build_charset(charset: &str) -> Vec<CharBitmap> {
    charset.chars().filter_map(|ch| {
        let glyph = BASIC_FONTS.get(ch)
            .or_else(|| BLOCK_FONTS.get(ch)); 

        if let Some(glyph) = glyph {
            let mut bitmap = vec![vec![0u8; 8]; 8];

            for (y, &byte) in glyph.iter().enumerate() {
                for x in 0..8 {
                    if (byte >> x) & 1 == 1 {
                        bitmap[y][x] = 255;
                    }
                }
            }

            let intensity = compute_intensity(&bitmap);
            Some(CharBitmap { ch, bitmap, intensity })
        } else {
            eprintln!("Warning: Character '{}' not found in font8x8", ch);
            None 
        }
    }).collect()
}

fn compute_intensity(bitmap: &[Vec<u8>]) -> f32 {
    let sum: u32 = bitmap.iter()
        .flat_map(|r| r.iter())
        .map(|&v| v as u32)
        .sum();
    sum as f32 / 64.0 
}