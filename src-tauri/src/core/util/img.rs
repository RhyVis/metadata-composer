use crate::api::get_client;
use crate::core::util::config::get_config;
use anyhow::{Result, anyhow};
use image::{DynamicImage, ImageFormat};
use image_hasher::HasherConfig;
use log::info;
use std::path::Path;

pub async fn process_image_web(url: &str) -> Result<String> {
    let client = get_client();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to fetch image from URL: {}, {}",
            url,
            response.status().as_str()
        ));
    }

    let image_bytes = response.bytes().await?;
    let img = image::load_from_memory(&image_bytes)?;
    process_image_internal(img)
}

pub fn process_image(source: impl AsRef<Path>) -> Result<String> {
    let source = source.as_ref();
    if !source.exists() {
        return Err(anyhow!("Source image does not exist: {}", source.display()));
    }
    let img = image::open(source)?;
    process_image_internal(img)
}

fn process_image_internal(img: DynamicImage) -> Result<String> {
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

    let mut target = get_config()?.dir_image();
    target.push(format!("{hash_str}.png"));
    if !target.exists() {
        img.save_with_format(&target, ImageFormat::Png)?;
        info!("Processed image saved to: {}", target.display());
    } else {
        info!("Image already exists, skipping: {}", target.display());
    }

    Ok(hash_str)
}
