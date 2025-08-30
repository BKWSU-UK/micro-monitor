use std::env;

pub struct Config {
    pub website: String,
    pub email_recipients: Vec<String>,
    pub email_host: String,
    pub email_port: u16,
    pub email_username: String,
    pub email_password: String,
    pub email_ssl_enable: bool,
    pub email_ssl_protocols: String,
    pub email_from: String,
    pub email_subject: String,
    pub email_html: String,
}

impl Config {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let website = env::var("WEBSITE")?;
        let email_recipients_string = env::var("EMAIL_RECIPIENTS")?;
        let email_host = env::var("EMAIL_HOST")?;
        let email_port = env::var("EMAIL_PORT")?.parse::<u16>()?;
        let email_ssl_enable = env::var("EMAIL_SSL_ENABLE")
            .map(|s| s == "true" || s == "1")?;
        let email_ssl_protocols = env::var("EMAIL_SSL_PROTOCOLS")?;
        let email_username = env::var("EMAIL_USERNAME")?;
        let email_password = env::var("EMAIL_PASSWORD")?;
        let email_from = env::var("EMAIL_FROM")?;
        let email_subject = env::var("EMAIL_SUBJECT")?;
        let email_html = env::var("EMAIL_HTML")?;
        Ok(Config {
            website,
            email_recipients: email_recipients_string.split(",").map(|s| s.trim().to_string()).collect(),
            email_host,
            email_port,
            email_username,
            email_password,
            email_ssl_enable,
            email_ssl_protocols,
            email_from,
            email_subject,
            email_html
        })
    }
}