use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};
use std::path::Path;

pub fn load_image(path: &str) -> Result<DynamicImage> {
    let path = Path::new(path);

    if !path.exists() {
        anyhow::bail!("Input file '{}' not found", path.display());
    }

    let img = image::open(path)
        .with_context(|| format!("Failed to load image: {}", path.display()))?;

    Ok(img)
}

pub fn save_image(
    image: &DynamicImage,
    path: &str,
    format: ImageFormat,
    _quality: u8,
) -> Result<()> {
    let path = Path::new(path);

    // Create parent directory if needed
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    // Save the image
    // Note: quality is only used for JPEG, ignored for other formats
    match format {
        ImageFormat::Jpeg => {
            // Convert to RGB for JPEG (no alpha)
            let rgb = image.to_rgb8();
            rgb.save_with_format(path, format)
                .with_context(|| format!("Failed to save image: {}", path.display()))?;
        }
        _ => {
            image
                .save_with_format(path, format)
                .with_context(|| format!("Failed to save image: {}", path.display()))?;
        }
    }

    Ok(())
}

pub fn get_output_paths(
    input_path: &str,
    output_dir: &str,
    format: &str,
) -> (String, String, String) {
    let input_name = Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let ext = match format {
        "jpg" | "jpeg" => "jpg",
        _ => format,
    };

    let height_path = format!("{}/{}_height.{}", output_dir, input_name, ext);
    let normal_path = format!("{}/{}_normal.{}", output_dir, input_name, ext);
    let metallic_path = format!("{}/{}_metallic.{}", output_dir, input_name, ext);

    (height_path, normal_path, metallic_path)
}

/// Convert height map (f32) to grayscale image
pub fn height_to_image(width: u32, height: u32, data: &[f32]) -> DynamicImage {
    use image::{ImageBuffer, Luma};

    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let idx = (y * width + x) as usize;
        let value = (data[idx] * 255.0).clamp(0.0, 255.0) as u8;
        *pixel = Luma([value]);
    }

    DynamicImage::ImageLuma8(img)
}

/// Convert normal map (RGBA8) to RGB image
pub fn normal_to_image(width: u32, height: u32, data: &[u8]) -> DynamicImage {
    use image::{ImageBuffer, Rgb};

    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let idx = ((y * width + x) * 4) as usize;
        let r = data[idx];
        let g = data[idx + 1];
        let b = data[idx + 2];
        *pixel = Rgb([r, g, b]);
    }

    DynamicImage::ImageRgb8(img)
}

/// Convert metallic map (R8) to grayscale image
pub fn metallic_to_image(width: u32, height: u32, data: &[u8]) -> DynamicImage {
    use image::{ImageBuffer, Luma};

    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let idx = (y * width + x) as usize;
        *pixel = Luma([data[idx]]);
    }

    DynamicImage::ImageLuma8(img)
}

/// Map OutputFormat to ImageFormat
pub fn output_format_to_image_format(format: &super::cli::OutputFormat) -> ImageFormat {
    match format {
        super::cli::OutputFormat::Png => ImageFormat::Png,
        super::cli::OutputFormat::Jpg => ImageFormat::Jpeg,
        super::cli::OutputFormat::Tga => ImageFormat::Tga,
        super::cli::OutputFormat::Exr => ImageFormat::OpenExr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_image_not_found() {
        let result = load_image("nonexistent.png");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_get_output_paths() {
        let (h, n, m) = get_output_paths("textures/brick.png", "./output", "png");
        assert_eq!(h, "./output/brick_height.png");
        assert_eq!(n, "./output/brick_normal.png");
        assert_eq!(m, "./output/brick_metallic.png");
    }

    #[test]
    fn test_get_output_paths_jpg() {
        let (h, n, m) = get_output_paths("textures/brick.png", "./output", "jpg");
        assert_eq!(h, "./output/brick_height.jpg");
        assert_eq!(n, "./output/brick_normal.jpg");
        assert_eq!(m, "./output/brick_metallic.jpg");
    }

    #[test]
    fn test_height_to_image() {
        let data = vec![0.0f32, 0.5, 1.0, 0.25];
        let img = height_to_image(2, 2, &data);
        assert_eq!(img.width(), 2);
        assert_eq!(img.height(), 2);

        // Check pixel values (scaled to 0-255, truncation)
        let luma = img.to_luma8();
        assert_eq!(luma.get_pixel(0, 0)[0], 0);
        assert_eq!(luma.get_pixel(1, 0)[0], 127); // 0.5 * 255 truncated
        assert_eq!(luma.get_pixel(0, 1)[0], 255);
    }
}
