#![allow(dead_code)]

use image::DynamicImage;
use std::path::Path;

pub fn recolor_image(_source: &Path, _dest: &Path, _r: u8, _g: u8, _b: u8) -> Result<(), String> {
    Err("recolor_image is deprecated, use ImageMagick CLI instead".into())
}

pub fn recolor_dynamic_image(_img: &DynamicImage, _r: u8, _g: u8, _b: u8) -> DynamicImage {
    unimplemented!("recolor_dynamic_image is deprecated")
}