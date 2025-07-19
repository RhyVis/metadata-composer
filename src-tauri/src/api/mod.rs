#![allow(dead_code)]

use anyhow::{Result, anyhow};
use reqwest::Client;
use std::sync::OnceLock;
use std::time::Duration;

pub mod dl_site;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn init_client() -> Result<()> {
    fn create_client() -> Result<Client> {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .tcp_keepalive(Some(Duration::from_secs(30)))
            .connection_verbose(true)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))
    }

    CLIENT
        .set(create_client()?)
        .map_err(|_| anyhow!("HTTP client already initialized"))?;

    Ok(())
}

pub fn get_client() -> &'static Client {
    #[cfg(debug_assertions)]
    {
        if CLIENT.get().is_none() {
            init_client().unwrap();
        }
    }

    CLIENT
        .get()
        .expect("HTTP client not initialized. Call `init_client()` first.")
}
