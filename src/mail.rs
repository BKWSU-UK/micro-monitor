use crate::Config;
use lettre::message::{header, Mailbox, Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Transport, SmtpTransport};
use regex::Regex;
use anyhow::Result;

fn strip_tags_regex(html: &str) -> String {
    // (?s) dot matches newlines, (?i) case-insensitive
    // Also drop comments and the *contents* of script/style
    let re = Regex::new(r"(?is)<!--.*?-->|<script\b[^>]*>.*?</script>|<style\b[^>]*>.*?</style>|<[^>]+>").unwrap();
    re.replace_all(html, "").to_string()
}

fn replace_website(text: &str, website: &str) -> String {
    let re = Regex::new(r"\{website\}").unwrap();
    re.replace_all(text, website).to_string()
}

fn build_html_email(config: &Config) -> anyhow::Result<Message> {
    let from = config.email_from.as_str();
    let website = config.website.as_str();
    let subject = replace_website(config.email_subject.as_str(), website);
    let from_mb: Mailbox = from.parse()?;
    let html = replace_website(config.email_html.as_str(), website);
    let mut builder = Message::builder()
        .from(from_mb)
        .subject(subject);
    for to in config.email_recipients.clone() {
        builder = builder.to(to.as_str().parse()?);
    }
    let msg = builder
        .multipart(
            MultiPart::alternative() // text/plain + text/html
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(strip_tags_regex(html.as_str()).to_owned()),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(html.to_owned()),
                ),
        )?;
    Ok(msg)
}

/// Build an SMTP transport that enforces TLS (TLSv1.2+ with rustls).
fn make_mailer(
    config: &Config
) -> Result<SmtpTransport> {
    let smtp_host = config.email_host.clone();
    let username: &str = config.email_username.as_str();
    let password: &str = config.email_password.as_str();

    let creds = Credentials::new(username.to_string(), password.to_string());

    let smtp_transport: SmtpTransport = SmtpTransport::starttls_relay(smtp_host.as_str())
        .unwrap()  // Unwrap the Result, panics in case of error
        .credentials(creds)  // Provide the credentials to the transport
        .build();

    Ok(smtp_transport)
}

pub fn send_html(
    config: &Config
) -> anyhow::Result<()> {
    let mailer = make_mailer(config)?;
    let email = build_html_email(config)?;
    mailer.send(&email)?;
    Ok(())
}