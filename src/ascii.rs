use crate::image::BlockSample;
use crate::font::CharBitmap;

pub fn render(blocks: Vec<Vec<BlockSample>>, charset: &[CharBitmap], color_enabled: bool) -> String {
    let mut ascii_art = String::new();

    for row in blocks.iter() {
        for block in row.iter() {
            let ch = match_char(block.lum, charset);

            if color_enabled {
                ascii_art.push_str(&format!(
                    "\x1b[38;2;{};{};{}m{}\x1b[0m",
                    block.r, block.g, block.b, ch
                ));
            } else {
                ascii_art.push(ch);
            }
        }
        ascii_art.push('\n');
    }

    ascii_art
}


fn match_char(intensity: f32, charset: &[CharBitmap]) -> char {
    if charset.is_empty() {
        return ' ';
    }

    let idx = ((intensity / 255.0) * (charset.len() as f32 - 1.0)).round() as usize;
    charset[idx].ch
}
