use image::DynamicImage;
use std::path::Path;

pub fn recolor_image(source: &Path, dest: &Path, r: u8, g: u8, b: u8) -> Result<(), String> {
    let img = image::open(source).map_err(|e| format!("Failed to open image: {}", e))?;

    let recolored = recolor_dynamic_image(&img, r, g, b);

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    recolored
        .save(dest)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(())
}

pub fn recolor_dynamic_image(img: &DynamicImage, r: u8, g: u8, b: u8) -> DynamicImage {
    let mut result = img.to_rgba8();

    for (_x, _y, pixel) in result.enumerate_pixels_mut() {
        let lum = luminance(pixel[0], pixel[1], pixel[2]);

        let blend = |base: u8, color: u8| -> u8 {
            let blended = (base as f32 * 0.3 + color as f32 * 0.7) as u8;
            blended
        };

        pixel[0] = blend(lum, r);
        pixel[1] = blend(lum, g);
        pixel[2] = blend(lum, b);
        pixel[3] = pixel[3];
    }

    DynamicImage::ImageRgba8(result)
}

fn luminance(r: u8, g: u8, b: u8) -> u8 {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    ((0.299 * r + 0.587 * g + 0.114 * b) * 255.0) as u8
}

pub fn walkdir(dir: &Path) -> Result<Vec<std::fs::DirEntry>, String> {
    let mut entries = Vec::new();
    walkdir_recursive_internal(dir, &mut entries)?;
    Ok(entries)
}

fn walkdir_recursive_internal(dir: &Path, entries: &mut Vec<std::fs::DirEntry>) -> Result<(), String> {
    for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            walkdir_recursive_internal(&path, entries)?;
        } else {
            entries.push(entry);
        }
    }
    Ok(())
}