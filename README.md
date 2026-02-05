# CAPTCHA Generator

A Rust library for generating customizable CAPTCHA images with distortion and noise effects.

[![Crates.io](https://img.shields.io/crates/v/captcha-generator.svg)](https://crates.io/crates/captcha-generator)
[![Documentation](https://docs.rs/captcha-generator/badge.svg)](https://docs.rs/captcha-generator)

## Features

- 🎨 Centered, readable text with customizable font size
- 🔀 Random rotation and positioning for security
- 🌊 Wave distortion effects
- 📊 Interference lines and noise dots
- ⚙️ Fully configurable generation parameters
- 🚀 Easy to use API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
captcha-generator = "0.1"
```

## Usage

### Basic Usage

```rust
use captcha_generator::Captcha;

fn main() {
    // Generate a CAPTCHA with default settings
    let captcha = Captcha::new();
    
    println!("Code: {}", captcha.code);
    
    // Save to file
    captcha.save("captcha.png").unwrap();
}
```

### Custom Configuration

```rust
use captcha_generator::{Captcha, CaptchaConfig};

fn main() {
    let config = CaptchaConfig {
        width: 300,
        height: 120,
        code_length: 8,
        font_size: 60.0,
        interference_lines: (3, 5),
        noise_dots: 150,
        wave_amplitude: (2.0, 3.0),
    };
    
    let captcha = Captcha::with_config(config);
    captcha.save("custom_captcha.png").unwrap();
}
```

### Get PNG Bytes (for web servers)

```rust
use captcha_generator::Captcha;

fn main() {
    let captcha = Captcha::new();
    let png_bytes = captcha.to_png_bytes().unwrap();
    
    // Use png_bytes to send over HTTP, etc.
}
```

## Configuration Options

| Parameter | Default | Description |
|-----------|---------|-------------|
| `width` | 280 | Image width in pixels |
| `height` | 100 | Image height in pixels |
| `code_length` | 6 | Length of the CAPTCHA code |
| `font_size` | 52.0 | Font size for the text |
| `interference_lines` | (2, 4) | Min and max number of interference lines |
| `noise_dots` | 100 | Number of random noise dots |
| `wave_amplitude` | (1.5, 2.5) | Min and max wave distortion amplitude |

## Command Line Usage

After installation, you can also use the binary:

```bash
cargo run --bin captcha-gen
```

This will generate a `captcha.png` file in the current directory.

## Example

The generated CAPTCHA will look like centered text with:
- Random rotation per character
- Wave distortion effects
- Curved interference lines
- Random noise dots
- Light gradient background

## Requirements

The library includes an embedded DejaVu Sans font, so no external font files are needed.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
