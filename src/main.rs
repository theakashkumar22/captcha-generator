use captcha_generator::Captcha;

fn main() {
    // Generate a CAPTCHA with default settings
    let captcha = Captcha::new();

    println!("Generated CAPTCHA code: {}", captcha.code);

    // Save the image
    match captcha.save("captcha.png") {
        Ok(_) => println!("CAPTCHA saved as captcha.png"),
        Err(e) => eprintln!("Failed to save CAPTCHA: {}", e),
    }
}
