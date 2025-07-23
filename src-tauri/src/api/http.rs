use anyhow::{Result, anyhow};
use reqwest::{Response, Url};

use crate::api::get_client;

/// Using [reqwest] to fetch a URL.
pub async fn fetch_url(url: &str) -> Result<Response> {
    let url_parse = Url::parse(&url).map_err(|e| anyhow!("Failed to parse URL {}: {}", url, e))?;
    get_client()
        .get(url_parse)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to fetch URL {}: {}", url, e))
}
