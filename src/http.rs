use std::time::Duration;
use reqwest::{Client, StatusCode};

use crate::Config;

pub struct Health {
    pub reachable: bool,
    pub healthy: bool,
}

pub async fn check_url(config: &Config) -> Result<Health, reqwest::Error> {
    let url = config.website.clone();
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::limited(10))
        .user_agent(&url)
        .build()?;

    // Prefer HEAD to avoid downloading bodies
    let head = client.head(&url).send().await;

    let resp = match head {
        Ok(r) => {
            if r.status() == StatusCode::METHOD_NOT_ALLOWED || r.status() == StatusCode::NOT_IMPLEMENTED {
                let get_res = client.get(&url).send().await?;
                get_res
            } else {
                r
            }
        }
        Err(_) => {
            // Try one GET in case the failure was HEAD-specific (proxies/CDNs)
            match client.get(&url).send().await {
                Ok(r) => r,
                Err(e) => return Ok(Health { reachable: false, healthy: false }), // timeout/DNS/TLS/etc.
            }
        }
    };

    let status = resp.status();
    Ok(
        Health { reachable: true, healthy: status.is_success() }
    )
}