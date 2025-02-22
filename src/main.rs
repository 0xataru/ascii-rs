use std::error::Error;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use image::imageops::FilterType;
use image::GenericImageView;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: cargo run -- <path_to_file>".into());
    }
    
    let input_file = &args[1];
    let output_file = "output.txt";

    if !Path::new(input_file).exists() {
        return Err(format!("File {} does not exist", input_file).into());
    }

    let img = image::open(input_file)?;
    let ascii_art = convert_to_ascii(&img, 100);

    let mut file = File::create(output_file).map_err(handle_error)?;
    file.write_all(ascii_art.as_bytes()).map_err(handle_error)?;

    println!("ASCII art saved to {}", output_file);

    Ok(())
}

fn convert_to_ascii(img: &image::DynamicImage, width: u32) -> String {
    let ascii_chars = " .:-=+*#%@";
    let (img_width, img_height) = img.dimensions();
    let aspect_ratio = img_width as f32 / img_height as f32;
    let height = (width as f32 * aspect_ratio * 0.55) as u32;

    let resized = img.resize(width, height, FilterType::Nearest);

    let gray = resized.to_luma8();
    let mut result = String::new();

        for y in 0..gray.height() {
        for x in 0..gray.width() {
            let pixel = gray.get_pixel(x, y);
            let intensity = pixel[0];
            
            let char_index = (intensity as f32 / 255.0 * (ascii_chars.len() - 1) as f32) as usize;
            let ascii_char = ascii_chars.chars().nth(char_index).unwrap();
            result.push(ascii_char);
        }
        result.push('\n');
    }

    result
}

fn handle_error(e: std::io::Error) -> Box<dyn std::error::Error> {
    eprintln!("Error: {}", e);
    Box::new(e)
}