use captcha_generator::{Captcha, CaptchaConfig};

fn main() {
    println!("=== CAPTCHA Generator Examples ===\n");

    // Example 1: Default CAPTCHA
    println!("1. Creating default CAPTCHA...");
    let captcha1 = Captcha::new();
    println!("   Code: {}", captcha1.code);
    captcha1.save("examples/default_captcha.png").unwrap();
    println!("   Saved to: examples/default_captcha.png\n");

    // Example 2: Custom size
    println!("2. Creating large CAPTCHA...");
    let config2 = CaptchaConfig {
        width: 400,
        height: 150,
        font_size: 70.0,
        ..Default::default()
    };
    let captcha2 = Captcha::with_config(config2);
    println!("   Code: {}", captcha2.code);
    captcha2.save("examples/large_captcha.png").unwrap();
    println!("   Saved to: examples/large_captcha.png\n");

    // Example 3: Long code
    println!("3. Creating CAPTCHA with long code...");
    let config3 = CaptchaConfig {
        code_length: 10,
        width: 400,
        ..Default::default()
    };
    let captcha3 = Captcha::with_config(config3);
    println!("   Code: {}", captcha3.code);
    captcha3.save("examples/long_code_captcha.png").unwrap();
    println!("   Saved to: examples/long_code_captcha.png\n");

    // Example 4: High security (more interference)
    println!("4. Creating high-security CAPTCHA...");
    let config4 = CaptchaConfig {
        interference_lines: (5, 8),
        noise_dots: 200,
        wave_amplitude: (3.0, 4.0),
        ..Default::default()
    };
    let captcha4 = Captcha::with_config(config4);
    println!("   Code: {}", captcha4.code);
    captcha4.save("examples/high_security_captcha.png").unwrap();
    println!("   Saved to: examples/high_security_captcha.png\n");

    // Example 5: Minimal interference (easier to read)
    println!("5. Creating easy-to-read CAPTCHA...");
    let config5 = CaptchaConfig {
        interference_lines: (1, 2),
        noise_dots: 50,
        wave_amplitude: (0.5, 1.0),
        ..Default::default()
    };
    let captcha5 = Captcha::with_config(config5);
    println!("   Code: {}", captcha5.code);
    captcha5.save("examples/easy_captcha.png").unwrap();
    println!("   Saved to: examples/easy_captcha.png\n");

    // Example 6: Get PNG bytes (for web servers)
    println!("6. Getting PNG bytes for HTTP response...");
    let captcha6 = Captcha::new();
    let png_bytes = captcha6.to_png_bytes().unwrap();
    println!("   Code: {}", captcha6.code);
    println!("   PNG size: {} bytes", png_bytes.len());
    println!("   (Bytes can be sent directly in HTTP response)\n");

    println!("✓ All examples completed!");
}
