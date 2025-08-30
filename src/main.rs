use dotenvy::dotenv;

use config::Config;
use logger::init_tracing;
use tracing::{info, error};
use http::check_url;
use mail::send_html;

mod config;
mod logger;
mod http;
mod mail;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    init_tracing();
    dotenv()?;
    let config = Config::init()?;
    info!("Configuration read successfully.");
    let health = check_url(&config).await;
    match health {
        Ok(r) => {
            info!("Website: {}", config.website);
            info!("Health status: reachable: {}, healthy: {}", r.reachable, r.healthy);
            if !r.reachable || !r.healthy {
                let mail_res = send_html(&config);
                match mail_res {
                    Ok(_res) => {
                        info!("Notification sent");
                    }
                    Err(e) => {
                        error!(error = ?e, "Failed to send email.");
                    }
                }
            }
        }
        Err(e) => {
            error!(error = ?e, "Health check failed");
        }
    }
    Ok(())
}

