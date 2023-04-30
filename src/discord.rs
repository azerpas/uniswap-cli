use std::fmt;

use reqwest::{Error, StatusCode};
use serde::Serialize;

#[derive(Debug)]
pub enum WebhookError {
    ReqwestError(Error),
    NonSuccessStatus(StatusCode),
}

impl fmt::Display for WebhookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebhookError::ReqwestError(e) => write!(f, "Reqwest Error: {}", e),
            WebhookError::NonSuccessStatus(status) => {
                write!(f, "Non-success status code: {}", status)
            }
        }
    }
}

impl From<Error> for WebhookError {
    fn from(error: Error) -> Self {
        WebhookError::ReqwestError(error)
    }
}

#[derive(Serialize)]
pub struct WebhookMessage {
    pub content: String,
}

pub async fn send_message(webhook_url: &str, message: &WebhookMessage) -> Result<(), WebhookError> {
    let client = reqwest::Client::new();
    let response = client
        .post(webhook_url)
        .json(message)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(WebhookError::NonSuccessStatus(response.status()))
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_send_message() {
        dotenv().ok();
        let webhook_url = env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL not set");
        let message = WebhookMessage {
            content: String::from("Hello, Discord!"),
        };

        let result = send_message(&webhook_url, &message).await;
        assert!(result.is_ok());
    }
}
