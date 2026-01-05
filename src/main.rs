use clap::Parser;
use std::process;
use pixel2ascii::{ascii::render, cli::Cli, font::{build_charset}, image::{compute_block_size, load_rgb, sample_image_blocks}};

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.validate() {
        eprintln!("error: {}", e);
        process::exit(2);
    }

    // --no-color overrides --color
    let color_enabled = if cli.no_color { false } else { cli.color };

    // --bg only valid when color is enabled
    let use_background = cli.bg && color_enabled;

    // Aspect ratio: if --no-aspect set, use 1.0 (square blocks)
    let aspect = if cli.no_aspect { 1.0_f32 } else { cli.aspect };

    let charset = cli.resolved_charset();

    if !cli.quiet {
        eprintln!("Parsed CLI: {:?}", cli);
        eprintln!("Derived: color_enabled={}, use_background={}, aspect={}, charset_len={}",
            color_enabled, use_background, aspect, charset.len());
    }

    // load image using the image pipeline

    let image = load_rgb(cli.input.to_string_lossy().as_ref());
    
    // getting block size

    let (block_w, block_h) = compute_block_size(image.width(), image.height(), cli.width, aspect);

    // computing the block rgb and luminance (inverted also)

    let blocks = sample_image_blocks(&image, block_w, block_h, image.width(), image.height(), cli.invert);

    // computing the luminance of the charset
    let charset_string: String = charset.iter().collect();
    let char_bitmap = build_charset(&charset_string);

    let ascii_out = render(blocks, &char_bitmap, color_enabled);

    println!("{}", ascii_out);
}