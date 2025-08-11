use crate::domain::{
    entities::ImageData,
    value_objects::ConversionConfig,
};
use image::{DynamicImage, GenericImageView, GrayImage, Luma, Rgb, RgbImage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Failed to decode image: {0}")]
    ImageDecodeError(#[from] image::ImageError),
    #[error("Invalid image data")]
    InvalidImageData,
}

/// Service for converting images to ASCII art
pub struct AsciiConversionService;

impl AsciiConversionService {
    /// Create a new ASCII conversion service
    pub fn new() -> Self {
        Self
    }

    /// Convert image data to ASCII art
    pub async fn convert_to_ascii(
        &self,
        image_data: &ImageData,
        config: &ConversionConfig,
    ) -> Result<String, ConversionError> {
        // Load image from bytes
        let img = image::load_from_memory(&image_data.data)?;
        
        // Convert to ASCII using the improved algorithm
        let ascii_art = self.convert_image_to_ascii(&img, config).await;
        
        Ok(ascii_art)
    }

    /// Convert DynamicImage to ASCII art with improved algorithm
    async fn convert_image_to_ascii(
        &self,
        img: &DynamicImage,
        config: &ConversionConfig,
    ) -> String {
        let ascii_chars = config.detail_level.char_set();

        let (img_width, img_height) = img.dimensions();
        let aspect_ratio = img_height as f32 / img_width as f32;
        // Adjust aspect ratio compensation for better proportions in text
        let height = (config.width as f32 * aspect_ratio * 0.43) as u32;

        // Use better resampling for sharper results
        let resized = img.resize_exact(config.width, height, image::imageops::FilterType::CatmullRom);
        
        // Apply contrast enhancement before converting to grayscale
        let contrast_adjusted = self.enhance_contrast(&resized, config.contrast_factor);
        let gray = contrast_adjusted.to_luma8();

        // Apply edge-preserving smoothing to reduce noise while maintaining details
        let smoothed = self.gaussian_blur(&gray, config.blur_sigma);
        
        // Use adaptive thresholding for better character mapping
        let processed = self.adaptive_threshold(&smoothed, ascii_chars.len());

        let mut result = String::with_capacity((config.width * height + height) as usize);

        for (y, row) in processed.rows().enumerate() {
            for pixel in row {
                let Luma([intensity]) = *pixel;
                let char_index = self.map_intensity_to_char(intensity, ascii_chars.len());
                result.push(ascii_chars.chars().nth(char_index).unwrap());
            }
            if y < height as usize - 1 {
                result.push('\n');
            }
        }

        result
    }

    /// Enhance contrast of an image
    fn enhance_contrast(&self, img: &DynamicImage, factor: f32) -> DynamicImage {
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let mut enhanced = RgbImage::new(width, height);

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let Rgb([r, g, b]) = *pixel;
            
            // Apply contrast enhancement to each channel
            let new_r = ((r as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;
            let new_g = ((g as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;
            let new_b = ((b as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;
            
            enhanced.put_pixel(x, y, Rgb([new_r, new_g, new_b]));
        }

        DynamicImage::ImageRgb8(enhanced)
    }

    /// Apply Gaussian blur to reduce noise while preserving edges
    fn gaussian_blur(&self, img: &GrayImage, sigma: f32) -> GrayImage {
        if sigma <= 0.0 {
            return img.clone();
        }

        let (width, height) = img.dimensions();
        let mut result = img.clone();

        // Create Gaussian kernel
        let kernel_size = (6.0 * sigma).ceil() as i32;
        let kernel_size = if kernel_size % 2 == 0 { kernel_size + 1 } else { kernel_size };
        let half_kernel = kernel_size / 2;

        let mut kernel = Vec::new();
        let mut sum = 0.0;

        for i in -half_kernel..=half_kernel {
            let value = (-0.5 * (i as f32 / sigma).powi(2)).exp();
            kernel.push(value);
            sum += value;
        }

        // Normalize kernel
        for value in &mut kernel {
            *value /= sum;
        }

        // Horizontal pass
        let mut temp = GrayImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let mut weighted_sum = 0.0;
                let mut weight_sum = 0.0;

                for (i, &weight) in kernel.iter().enumerate() {
                    let offset_x = x as i32 + i as i32 - half_kernel;
                    if offset_x >= 0 && offset_x < width as i32 {
                        let pixel_value = img.get_pixel(offset_x as u32, y)[0] as f32;
                        weighted_sum += pixel_value * weight;
                        weight_sum += weight;
                    }
                }

                let new_value = if weight_sum > 0.0 {
                    (weighted_sum / weight_sum).clamp(0.0, 255.0) as u8
                } else {
                    img.get_pixel(x, y)[0]
                };
                
                temp.put_pixel(x, y, Luma([new_value]));
            }
        }

        // Vertical pass
        for y in 0..height {
            for x in 0..width {
                let mut weighted_sum = 0.0;
                let mut weight_sum = 0.0;

                for (i, &weight) in kernel.iter().enumerate() {
                    let offset_y = y as i32 + i as i32 - half_kernel;
                    if offset_y >= 0 && offset_y < height as i32 {
                        let pixel_value = temp.get_pixel(x, offset_y as u32)[0] as f32;
                        weighted_sum += pixel_value * weight;
                        weight_sum += weight;
                    }
                }

                let new_value = if weight_sum > 0.0 {
                    (weighted_sum / weight_sum).clamp(0.0, 255.0) as u8
                } else {
                    temp.get_pixel(x, y)[0]
                };
                
                result.put_pixel(x, y, Luma([new_value]));
            }
        }

        result
    }

    /// Apply adaptive thresholding for better character mapping
    fn adaptive_threshold(&self, img: &GrayImage, levels: usize) -> GrayImage {
        let (width, height) = img.dimensions();
        let mut result = img.clone();

        // Calculate histogram
        let mut histogram = vec![0; 256];
        for y in 0..height {
            for x in 0..width {
                let intensity = img.get_pixel(x, y)[0] as usize;
                histogram[intensity] += 1;
            }
        }

        // Calculate cumulative distribution
        let total_pixels = (width * height) as f32;
        let mut cumulative = vec![0.0; 256];
        cumulative[0] = histogram[0] as f32 / total_pixels;
        
        for i in 1..256 {
            cumulative[i] = cumulative[i - 1] + histogram[i] as f32 / total_pixels;
        }

        // Apply histogram equalization with level quantization
        for y in 0..height {
            for x in 0..width {
                let intensity = img.get_pixel(x, y)[0] as usize;
                let equalized = (cumulative[intensity] * 255.0) as u8;
                
                // Quantize to the specified number of levels
                let level = (equalized as f32 / 255.0 * (levels - 1) as f32).round() as usize;
                let quantized = (level as f32 / (levels - 1) as f32 * 255.0) as u8;
                
                result.put_pixel(x, y, Luma([quantized]));
            }
        }

        result
    }

    /// Map intensity value to character index using perceptual weighting
    fn map_intensity_to_char(&self, intensity: u8, char_count: usize) -> usize {
        // Apply gamma correction for better perceptual mapping
        let normalized = intensity as f32 / 255.0;
        let gamma_corrected = normalized.powf(0.7); // Slightly darken midtones
        let char_index = (gamma_corrected * (char_count - 1) as f32).round() as usize;
        char_index.min(char_count - 1)
    }
}

impl Default for AsciiConversionService {
    fn default() -> Self {
        Self::new()
    }
}
