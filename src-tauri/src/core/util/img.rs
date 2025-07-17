use crate::core::util::config::get_config;
use anyhow::{Result, anyhow};
use image::ImageFormat;
use image_hasher::HasherConfig;
use log::info;
use std::path::Path;

pub fn process_image(source: impl AsRef<Path>) -> Result<String> {
    let source = source.as_ref();
    if !source.exists() {
        return Err(anyhow!("Source image does not exist: {}", source.display()));
    }

    let mut target = get_config()?.dir_image();
    let img = image::open(&source)?;

    let width = img.width();
    let height = img.height();
    let max_size = 1000;

    let img = if width > max_size || height > max_size {
        let ratio_w = max_size as f32 / width as f32;
        let ratio_h = max_size as f32 / height as f32;
        let ratio = ratio_w.min(ratio_h);

        let new_width = (width as f32 * ratio) as u32;
        let new_height = (height as f32 * ratio) as u32;

        img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };

    let hasher = HasherConfig::new().to_hasher();
    let hash = hasher.hash_image(&img);
    let hash_str = hash.to_base64();

    target.push(format!("{hash_str}.png"));
    if !target.exists() {
        img.save_with_format(&target, ImageFormat::Png)?;
        info!("Processed image saved to: {}", target.display());
    } else {
        info!("Image already exists, skipping: {}", target.display());
    }

    Ok(hash_str)
}
