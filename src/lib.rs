use std::{env, fs::File, io::Write, path::Path, time::Instant};

use image::{DynamicImage, GenericImageView, GrayImage, Luma};

/// Run the program
/// 
/// # Arguments
/// 
/// * `input_path` - The path to the input image
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <path_to_image>", args[0]);
        return Err("path to image not specified".into());
    }
    
    let input_path = &args[1];
    let output_path = "output.txt";
    
    if !Path::new(input_path).exists() {
        return Err(format!("file {} does not exist", input_path).into());
    }
    
    let img = image::open(input_path)?;
    let ascii_art = convert_to_ascii(&img, 100, true);
    
    let mut output_file = File::create(output_path)?;
    output_file.write_all(ascii_art.as_bytes())?;
    
    let duration = start_time.elapsed();
    println!("ASCII-art successfully created in {} in {:.2?}", output_path, duration);
    Ok(())
}

/// Convert an image to ASCII art
/// 
/// # Arguments
/// 
/// * `img` - The image to convert
/// * `width` - The width of the output ASCII art
/// * `detail` - Whether to use a more detailed ASCII art
/// 
/// # Returns
/// 
/// * `String` - The ASCII art
fn convert_to_ascii(img: &DynamicImage, width: u32, detail: bool) -> String {
    let ascii_chars = if detail {
        " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"
    } else {
        " .:-=+*#%@"
    };
    
    let (img_width, img_height) = img.dimensions();
    let aspect_ratio = img_height as f32 / img_width as f32;
    let height = (width as f32 * aspect_ratio * 0.5) as u32;
    
    let resized = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let gray = resized.to_luma8();
    
    let dithered = floyd_steinberg_dithering(&gray, ascii_chars.len() as u8);
    
    let mut result = String::with_capacity((width * height + height) as usize);
    
    for (y, row) in dithered.rows().enumerate() {
        for pixel in row {
            let Luma([intensity]) = *pixel;
            let char_index = (intensity as f32 / 255.0 * (ascii_chars.len() - 1) as f32) as usize;
            result.push(ascii_chars.chars().nth(char_index).unwrap());
        }
        if y < height as usize - 1 {
            result.push('\n');
        }
    }
    
    result
}

/// Dither an image using the Floyd-Steinberg algorithm
/// 
/// # Arguments
/// 
/// * `img` - The image to dither
/// * `levels` - The number of levels in the dithering
/// 
/// # Returns
/// 
/// * `GrayImage` - The dithered image
fn floyd_steinberg_dithering(img: &GrayImage, levels: u8) -> GrayImage {
    let mut result = img.clone();
    let (width, height) = result.dimensions();
    
    let mut buffer: Vec<Vec<f32>> = vec![vec![0.0; width as usize]; height as usize];
    
    for y in 0..height {
        for x in 0..width {
            buffer[y as usize][x as usize] = result.get_pixel(x, y)[0] as f32;
        }
    }
    
    for y in 0..height {
        for x in 0..width {
            let old_pixel = buffer[y as usize][x as usize];
            let new_pixel = (old_pixel / 255.0 * (levels - 1) as f32).round() * (255.0 / (levels - 1) as f32);
            let error = old_pixel - new_pixel;
            
            buffer[y as usize][x as usize] = new_pixel;
            
            let pixels = [
                (x.wrapping_add(1), y, 7.0 / 16.0),
                (x.wrapping_sub(1), y.wrapping_add(1), 3.0 / 16.0),
                (x, y.wrapping_add(1), 5.0 / 16.0),
                (x.wrapping_add(1), y.wrapping_add(1), 1.0 / 16.0),
            ];

            for (px, py, factor) in pixels {
                if px < width && py < height {
                    buffer[py as usize][px as usize] = (buffer[py as usize][px as usize] + error * factor).clamp(0.0, 255.0);
                }
            }
        }
        
        for x in 0..width {
            result.put_pixel(x, y, Luma([buffer[y as usize][x as usize] as u8]));
        }
    }
    
    result
}

/// Handle an error
/// 
/// # Arguments
/// 
/// * `error` - The error to handle
/// 
/// # Returns
/// 
/// * `()` - The function returns an empty tuple
pub fn handle_error(error: Box<dyn std::error::Error>) {
    eprintln!("error: {}", error);
    let mut source = error.source();
    while let Some(cause) = source {
        eprintln!("caused by: {}", cause);
        source = cause.source();
    }
    std::process::exit(1);
}