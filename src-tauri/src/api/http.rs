use crate::api::get_client;
use anyhow::Result;
use reqwest::Response;

/// Using [reqwest] to fetch a URL.
pub async fn fetch_url(url: &str) -> Result<Response> {
    get_client()
        .get(url)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch URL {}: {}", url, e))
}
