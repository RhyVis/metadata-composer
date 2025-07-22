use crate::api::http::fetch_url;
use crate::core::util::config::get_config;
use anyhow::{Result, anyhow};
use image::{DynamicImage, ImageFormat, RgbaImage};
use image_hasher::HasherConfig;
use log::info;
use std::path::Path;
use tauri::async_runtime;
use tokio::fs as tfs;

pub async fn process_image_web(url: &str) -> Result<String> {
    let response = fetch_url(url).await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to fetch image from URL: {}, {}",
            url,
            response.status().as_str()
        ));
    }

    let image_bytes = response.bytes().await?;
    let img = image::load_from_memory(&image_bytes)?;

    process_image_internal(img).await
}

pub async fn process_image_file(source: impl AsRef<Path>) -> Result<String> {
    let source = source.as_ref().to_owned();
    if !tfs::try_exists(&source).await? {
        return Err(anyhow!("Source image does not exist: {}", source.display()));
    }
    let img = async_runtime::spawn_blocking(move || image::open(source)).await??;

    process_image_internal(img).await
}

pub async fn process_image_bytes(data: (Vec<u8>, u32, u32)) -> Result<String> {
    let (data, width, height) = data;
    let img = DynamicImage::ImageRgba8(
        async_runtime::spawn_blocking(move || {
            RgbaImage::from_raw(width, height, data)
                .ok_or_else(|| anyhow!("Failed to create image from raw data"))
        })
        .await??,
    );

    process_image_internal(img).await
}

async fn process_image_internal(img: DynamicImage) -> Result<String> {
    let width = img.width();
    let height = img.height();
    let max_size = 1000;

    let img = if width > max_size || height > max_size {
        let ratio_w = max_size as f32 / width as f32;
        let ratio_h = max_size as f32 / height as f32;
        let ratio = ratio_w.min(ratio_h);

        let new_width = (width as f32 * ratio) as u32;
        let new_height = (height as f32 * ratio) as u32;

        async_runtime::spawn_blocking(move || {
            img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
        })
        .await?
    } else {
        img
    };

    let hash_str = async_runtime::spawn_blocking(move || {
        let hasher = HasherConfig::new().to_hasher();
        let hash = hasher.hash_image(&img);
        let hash_str = hash.to_base64();
        (hash_str.replace('/', "_").replace('+', "-"), img)
    })
    .await?;
    let (hash_str, img) = hash_str;

    let mut target = get_config()?.dir_image();
    target.push(format!("{hash_str}.png"));

    info!("Saving processed image to: {}", target.display());

    if !tfs::try_exists(&target).await? {
        let target_inner = target.clone();
        async_runtime::spawn_blocking(move || {
            img.save_with_format(target_inner, ImageFormat::Png)
                .map_err(|e| anyhow!("Failed to save image: {}", e))
        })
        .await??;
        info!(
            "Image saved successfully {} to {}",
            hash_str,
            target.display()
        );
    } else {
        info!("Image already exists, skipping save: {}", hash_str);
    }

    Ok(hash_str)
}
