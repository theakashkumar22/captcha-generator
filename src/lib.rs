use image::{Rgb, RgbImage};
use rand::Rng;
use rusttype::{point, Font, Scale};

/// Embedded DejaVu Sans font
const FONT_DATA: &[u8] = include_bytes!("../assets/dejavusans.ttf");

/// Configuration for CAPTCHA generation
#[derive(Debug, Clone)]
pub struct CaptchaConfig {
    /// Width of the CAPTCHA image in pixels
    pub width: u32,
    /// Height of the CAPTCHA image in pixels
    pub height: u32,
    /// Length of the CAPTCHA code
    pub code_length: usize,
    /// Font size for the text
    pub font_size: f32,
    /// Number of interference lines (min, max)
    pub interference_lines: (usize, usize),
    /// Number of noise dots
    pub noise_dots: usize,
    /// Wave distortion amplitude range (min, max)
    pub wave_amplitude: (f32, f32),
}

impl Default for CaptchaConfig {
    fn default() -> Self {
        Self {
            width: 280,
            height: 100,
            code_length: 6,
            font_size: 52.0,
            interference_lines: (2, 4),
            noise_dots: 100,
            wave_amplitude: (1.5, 2.5),
        }
    }
}

/// A CAPTCHA image and its corresponding code
#[derive(Debug)]
pub struct Captcha {
    /// The generated code string
    pub code: String,
    /// The CAPTCHA image
    pub image: RgbImage,
}

impl Captcha {
    /// Generate a new CAPTCHA with default configuration
    pub fn new() -> Self {
        Self::with_config(CaptchaConfig::default())
    }

    /// Generate a new CAPTCHA with custom configuration
    pub fn with_config(config: CaptchaConfig) -> Self {
        let code = generate_code(config.code_length);
        let image = generate_captcha_image(&code, &config);

        Self { code, image }
    }

    /// Save the CAPTCHA image to a file
    pub fn save(&self, path: &str) -> Result<(), image::ImageError> {
        self.image.save(path)
    }

    /// Get the CAPTCHA image as PNG bytes
    pub fn to_png_bytes(&self) -> Result<Vec<u8>, image::ImageError> {
        let mut bytes = Vec::new();
        self.image.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )?;
        Ok(bytes)
    }
}

impl Default for Captcha {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a random CAPTCHA code
fn generate_code(len: usize) -> String {
    let mut rng = rand::thread_rng();
    // Use only readable characters (avoiding 0/O, 1/I/l, etc.)
    let charset = "23456789ABCDEFGHJKLMNPQRSTUVWXYZ";
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

/// Create a gradient background
fn create_background(width: u32, height: u32) -> RgbImage {
    let mut rng = rand::thread_rng();
    let mut img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let base = 245 + rng.gen_range(0..10);
            let r = base;
            let g = (base - rng.gen_range(0..5)).clamp(240, 255);
            let b = (base - rng.gen_range(0..5)).clamp(240, 255);
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    img
}

/// Parameters for drawing a character
struct CharDrawParams {
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
    color: [u8; 3],
}

/// Draw a single character with rotation and positioning
fn draw_character(img: &mut RgbImage, ch: char, params: CharDrawParams, font: &Font, scale: Scale) {
    let glyph = font.glyph(ch).scaled(scale);

    if let Some(bb) = glyph.exact_bounding_box() {
        let glyph = glyph.positioned(point(0.0, 0.0));

        glyph.draw(|gx, gy, v| {
            if v < 0.01 {
                return;
            }

            let cx = bb.width() / 2.0;
            let cy = bb.height() / 2.0;
            let gx_f = gx as f32 - cx;
            let gy_f = gy as f32 - cy;

            let cos_r = params.rotation.cos();
            let sin_r = params.rotation.sin();

            let rotated_x = gx_f * cos_r - gy_f * sin_r;
            let rotated_y = gx_f * sin_r + gy_f * cos_r;

            let final_x = (rotated_x + cx + params.x_offset + bb.min.x) as i32;
            let final_y = (rotated_y + cy + params.y_offset + bb.min.y) as i32;

            if final_x >= 0 && final_y >= 0 {
                let fx = final_x as u32;
                let fy = final_y as u32;

                if fx < img.width() && fy < img.height() {
                    let bg = img.get_pixel(fx, fy).0;

                    let alpha = v;
                    let r = (bg[0] as f32 * (1.0 - alpha) + params.color[0] as f32 * alpha) as u8;
                    let g = (bg[1] as f32 * (1.0 - alpha) + params.color[1] as f32 * alpha) as u8;
                    let b = (bg[2] as f32 * (1.0 - alpha) + params.color[2] as f32 * alpha) as u8;

                    img.put_pixel(fx, fy, Rgb([r, g, b]));
                }
            }
        });
    }
}

/// Draw the CAPTCHA text on the image
fn draw_text(img: &mut RgbImage, text: &str, font_size: f32) {
    let font = Font::try_from_bytes(FONT_DATA).expect("Error loading font");
    let mut rng = rand::thread_rng();

    let scale = Scale::uniform(font_size);
    let char_spacing = 8.0;
    let mut total_width = 0.0;

    for ch in text.chars() {
        let glyph = font.glyph(ch).scaled(scale);
        total_width += glyph.h_metrics().advance_width + char_spacing;
    }
    total_width -= char_spacing;

    let start_x = (img.width() as f32 - total_width) / 2.0;
    let base_y = (img.height() as f32 / 2.0) + (font_size / 3.0);

    let mut current_x = start_x;

    for ch in text.chars() {
        let glyph = font.glyph(ch).scaled(scale);
        let advance = glyph.h_metrics().advance_width;

        let rotation = rng.gen_range(-0.26..0.26);
        let y_offset = base_y + rng.gen_range(-5.0..5.0);
        let x_offset = current_x + rng.gen_range(-2.0..2.0);

        let color = [
            rng.gen_range(30..70),
            rng.gen_range(30..70),
            rng.gen_range(30..70),
        ];

        let params = CharDrawParams {
            x_offset,
            y_offset,
            rotation,
            color,
        };

        draw_character(img, ch, params, &font, scale);

        current_x += advance + char_spacing;
    }
}

/// Add curved interference lines to the image
fn add_interference_lines(img: &mut RgbImage, line_range: (usize, usize)) {
    let mut rng = rand::thread_rng();
    let width = img.width();
    let height = img.height();

    for _ in 0..rng.gen_range(line_range.0..line_range.1) {
        let color = Rgb([
            rng.gen_range(180..210),
            rng.gen_range(180..210),
            rng.gen_range(180..210),
        ]);

        let start_y = rng.gen_range(0..height) as f32;
        let amplitude = rng.gen_range(8.0..12.0);
        let frequency = rng.gen_range(0.02..0.04);
        let thickness = 1;

        for x in 0..width {
            let y = start_y + (x as f32 * frequency).sin() * amplitude;

            for dy in -thickness..=thickness {
                let py = (y as i32 + dy).max(0).min(height as i32 - 1) as u32;
                if x < width && py < height {
                    img.put_pixel(x, py, color);
                }
            }
        }
    }
}

/// Add random noise dots to the image
fn add_noise_dots(img: &mut RgbImage, count: usize) {
    let mut rng = rand::thread_rng();
    let width = img.width();
    let height = img.height();

    for _ in 0..count {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let color = if rng.gen_bool(0.5) {
            Rgb([
                rng.gen_range(200..230),
                rng.gen_range(200..230),
                rng.gen_range(200..230),
            ])
        } else {
            Rgb([
                rng.gen_range(80..140),
                rng.gen_range(80..140),
                rng.gen_range(80..140),
            ])
        };

        img.put_pixel(x, y, color);

        if rng.gen_bool(0.2) {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let nx = (x as i32 + dx).max(0).min(width as i32 - 1) as u32;
                    let ny = (y as i32 + dy).max(0).min(height as i32 - 1) as u32;
                    if rng.gen_bool(0.3) {
                        img.put_pixel(nx, ny, color);
                    }
                }
            }
        }
    }
}

/// Apply wave distortion to the image
fn add_wave_distortion(img: &mut RgbImage, amplitude_range: (f32, f32)) -> RgbImage {
    let mut rng = rand::thread_rng();
    let width = img.width();
    let height = img.height();
    let mut new_img = create_background(width, height);

    let amplitude = rng.gen_range(amplitude_range.0..amplitude_range.1);
    let frequency = rng.gen_range(0.06..0.09);

    for y in 0..height {
        for x in 0..width {
            let offset = (y as f32 * frequency).sin() * amplitude;
            let src_x = (x as i32 + offset as i32).max(0).min(width as i32 - 1) as u32;

            let pixel = img.get_pixel(src_x, y);
            new_img.put_pixel(x, y, *pixel);
        }
    }

    new_img
}

/// Generate a complete CAPTCHA image from a code string
fn generate_captcha_image(code: &str, config: &CaptchaConfig) -> RgbImage {
    let mut img = create_background(config.width, config.height);
    draw_text(&mut img, code, config.font_size);
    add_interference_lines(&mut img, config.interference_lines);
    add_noise_dots(&mut img, config.noise_dots);
    add_wave_distortion(&mut img, config.wave_amplitude)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_code() {
        let code = generate_code(6);
        assert_eq!(code.len(), 6);
        assert!(code
            .chars()
            .all(|c| "23456789ABCDEFGHJKLMNPQRSTUVWXYZ".contains(c)));
    }

    #[test]
    fn test_captcha_creation() {
        let captcha = Captcha::new();
        assert_eq!(captcha.code.len(), 6);
        assert_eq!(captcha.image.width(), 280);
        assert_eq!(captcha.image.height(), 100);
    }

    #[test]
    fn test_custom_config() {
        let config = CaptchaConfig {
            width: 300,
            height: 120,
            code_length: 8,
            ..Default::default()
        };
        let captcha = Captcha::with_config(config);
        assert_eq!(captcha.code.len(), 8);
        assert_eq!(captcha.image.width(), 300);
        assert_eq!(captcha.image.height(), 120);
    }
}
