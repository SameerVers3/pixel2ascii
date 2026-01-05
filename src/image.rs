use image::{RgbImage};

pub fn load_rgb(path: &str) -> RgbImage {
    image::open(path).unwrap().to_rgb8()
}

pub struct BlockSample {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub lum: f32,
}

pub fn block_color(img: &RgbImage, x0: u32, y0: u32, w: u32, h: u32, is_invert: bool) -> BlockSample {
    let mut r_sum = 0u32;
    let mut g_sum = 0u32;
    let mut b_sum = 0u32;
    let mut count = 0u32;

    for y in y0..(y0 + h).min(img.height()) {
        for x in x0..(x0 + w).min(img.width()) {
            let p = img.get_pixel(x, y);
            r_sum += p[0] as u32;
            g_sum += p[1] as u32;
            b_sum += p[2] as u32;
            count += 1;
        }
    }

    let r = (r_sum / count) as u8;
    let g = (g_sum / count) as u8;
    let b = (b_sum / count) as u8;

    let mut lum = 0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32;

    if is_invert {
        lum = 255.0 - lum;
    }

    BlockSample { r, g, b, lum }
}

pub fn compute_block_size(
    img_width: u32,
    _img_height: u32,
    ascii_width: u32,
    aspect: f32,
) -> (u32, u32) {
    assert!(ascii_width > 0, "ascii_width must be > 0");
    assert!(aspect > 0.0, "aspect must be > 0");

    // Width of each block in pixels
    let block_w = img_width as f32 / ascii_width as f32;

    // Height of each block, adjusted by aspect ratio
    let block_h = block_w / aspect;

    // Round to nearest integer pixels
    let block_w = block_w.max(1.0).round() as u32;
    let block_h = block_h.max(1.0).round() as u32;

    (block_w, block_h)
}

pub fn sample_image_blocks(
    img: &RgbImage,
    block_w: u32,
    block_h: u32,
    image_w: u32,
    image_h: u32, 
    is_invert: bool,
) -> Vec<Vec<BlockSample>> {
    let mut blocks: Vec<Vec<BlockSample>> = Vec::new();

    let mut y = 0;

    while y < image_h {
        let mut row: Vec<BlockSample> = Vec::new();

        let mut x = 0;

        while x < image_w {
            let block = block_color(img, x, y, block_w, block_h, is_invert);
            row.push(block);

            x += block_w;
        }
        blocks.push(row);
        y += block_h;
    }

    blocks
}
