use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Write, stdout};
use std::process;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage};
use pixel2ascii::{ascii::render, cli::Cli, font::build_charset, image::{compute_block_size, is_gif, load_rgb, sample_image_blocks}};

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


    let is_gif = is_gif(&cli.input);

    // computing the luminance of the charset
    let charset_string: String = charset.iter().collect();
    let char_bitmap = build_charset(&charset_string);

    if is_gif {
        let file = BufReader::new(File::open(&cli.input).unwrap());
        let decoder = GifDecoder::new(file).unwrap();
        let frames = decoder.into_frames().collect_frames().unwrap();

        for frame in frames {
            let _delay = frame.delay();
            let rgba = frame.into_buffer();
            let rgb = DynamicImage::ImageRgba8(rgba).to_rgb8();

            let (block_w, block_h) = compute_block_size(rgb.width(), rgb.height(), cli.width, aspect);

            let blocks = sample_image_blocks(&rgb, block_w, block_h, rgb.width(), rgb.height(), cli.invert);
            let ascii_out = render(blocks, &char_bitmap, color_enabled);

            print!("\x1B[2J\x1B[H{}", ascii_out);
            stdout().flush().unwrap();
        }
    } else {
        // load image using the image pipeline

        let image = load_rgb(cli.input.to_string_lossy().as_ref());
        
        // getting block size

        let (block_w, block_h) = compute_block_size(image.width(), image.height(), cli.width, aspect);

        // computing the block rgb and luminance (inverted also)

        let blocks = sample_image_blocks(&image, block_w, block_h, image.width(), image.height(), cli.invert);

        let ascii_out = render(blocks, &char_bitmap, color_enabled);

        println!("{}", ascii_out);
    }
}