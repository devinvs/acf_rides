use reqwest::Client;
use std::error::Error;

const URL: &'static str = "https://api.mailgun.net/v3/rides.vstelt.dev/messages";
const FROM: &'static str = "ACF Rides <mail@rides.vstelt.dev>";
const TEMPLATE: &'static str = "rides_reset_pw";

pub async fn send_reset_email(to: &str, reset_id: &str) -> Result<(), Box<dyn Error>> {
    let key = std::env::var("MAILGUN_KEY")?;

    let params = [
        ("from", FROM),
        ("to", to),
        ("template", TEMPLATE),
        ("subject", "Reset Your Password"),
        ("t:variables", &format!("{{\"reset_id\": \"{}\"}}", reset_id))
    ];

    let client = Client::new();

    client.post(URL)
        .basic_auth("api", Some(key))
        .form(&params)
        .send().await?;

    Ok(())
}
