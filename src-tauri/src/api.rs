use std::{sync::OnceLock, time::Duration};

use anyhow::{Result, anyhow};
use reqwest::Client;

pub mod dl_site;
pub mod http;

static CLIENT: OnceLock<Client> = OnceLock::new();

/// Initializes the API components of the application.
pub fn init_api() -> Result<()> {
    init_client()?;
    Ok(())
}

fn init_client() -> Result<()> {
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

fn get_client() -> &'static Client {
    CLIENT.get().expect("HTTP client not initialized.")
}
