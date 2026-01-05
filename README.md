# pixel2ascii

Convert images into ASCII art with optional ANSI truecolor support.

![Rust](https://img.shields.io/badge/rust-stable-orange)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- **Image to ASCII conversion** - transforms any image into ASCII art
- **ANSI truecolor support** - colorized output for terminals that support 24-bit color
- **Multiple charset presets** - `default`, `dense`, and Unicode `blocks`
- **Custom charsets** - define your own dark-to-light character gradient
- **Aspect ratio correction** - accounts for non-square terminal characters
- **Luminance inversion** - invert for light-on-dark or dark-on-light output
- **Flexible output** - print to stdout or write to a file

## Installation

### From source

```bash
git clone https://github.com/SameerVers3/pixel2ascii.git
cd pixel2ascii
cargo build --release
```

The binary will be at `target/release/pixel2ascii`.

### From crates.io (coming soon)

```bash
cargo install pixel2ascii
```

## Usage

```bash
pixel2ascii <INPUT> [OPTIONS]
```

### Examples

Basic usage (grayscale, 100 characters wide):

```bash
pixel2ascii image.png
```

Colorized output at 120 characters wide:

```bash
pixel2ascii image.png --color --width 120
```

Use block characters for a bolder look:

```bash
pixel2ascii image.png --charset-preset blocks
```

Invert luminance (useful for light terminals):

```bash
pixel2ascii image.png --invert
```

Save output to a file:

```bash
pixel2ascii image.png -o output.txt
```

Custom charset (dark → light):

```bash
pixel2ascii image.png --charset "@#%*+=-:. "
```

## CLI Reference

| Flag / Option | Short | Default | Description |
|---------------|-------|---------|-------------|
| `<INPUT>` | | *required* | Path to input image file |
| `--width` | `-w` | `100` | Maximum ASCII character width |
| `--aspect` | | `0.5` | Character height/width ratio for aspect correction |
| `--no-aspect` | | | Disable aspect correction (square blocks) |
| `--invert` | | | Invert luminance mapping |
| `--color` | `-c` | | Enable ANSI truecolor foreground |
| `--bg` | | | Use background color mode (requires `--color`) |
| `--no-color` | | | Force grayscale output (overrides `--color`) |
| `--charset` | | | Custom charset ordered dark → light (≥2 chars) |
| `--charset-preset` | | `default` | Preset: `default`, `dense`, or `blocks` |
| `--output` | `-o` | | Write output to file instead of stdout |
| `--quiet` | | | Suppress non-error messages |
| `--help` | `-h` | | Show help |
| `--version` | `-V` | | Show version |

### Charset Presets

| Preset | Characters |
|--------|------------|
| `default` | `@%#*+=-:. ` |
| `dense` | `@M#W$9876543210?!abc;:+=-,._ ` |
| `blocks` | `█▓▒░ ` (Unicode block elements) |

## How It Works

1. **Load image** - uses the `image` crate to decode PNG, JPEG, GIF, BMP, etc.
2. **Compute block size** - divides the image into blocks based on target width and aspect ratio.
3. **Sample blocks** - computes average RGB and luminance for each block.
4. **Match characters** - maps luminance to the appropriate character from the charset.
5. **Render** - outputs plain text or ANSI-colored text depending on flags.

### Luminance Formula

```
L = 0.2126 × R + 0.7152 × G + 0.0722 × B
```

This is the standard Rec. 709 luminance formula.

## Project Structure

```
pixel2ascii/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs      # CLI entry point
    ├── lib.rs       # Library exports
    ├── cli.rs       # Clap-based argument parsing
    ├── image.rs     # Image loading and block sampling
    ├── font.rs      # Charset bitmap generation (font8x8)
    └── ascii.rs     # ASCII rendering logic
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | Command-line argument parsing |
| `image` | Image decoding (PNG, JPEG, etc.) |
| `font8x8` | 8×8 bitmap font for charset intensity calculation |

## License

MIT © [Sameer](https://github.com/SameerVers3)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

---

<p align="center">Made with ❤️ in Rust</p>
