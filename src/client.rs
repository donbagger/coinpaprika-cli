use anyhow::{bail, Result};
use reqwest::StatusCode;

pub struct ApiClient {
    http: reqwest::Client,
    coinpaprika_base: String,
    api_key: Option<String>,
}

impl ApiClient {
    pub fn new(api_key: Option<String>) -> Self {
        let coinpaprika_base = if api_key.is_some() {
            "https://api-pro.coinpaprika.com/v1".to_string()
        } else {
            "https://api.coinpaprika.com/v1".to_string()
        };

        Self {
            http: reqwest::Client::builder()
                .user_agent("coinpaprika-cli/0.1.0")
                .build()
                .expect("failed to build HTTP client"),
            coinpaprika_base,
            api_key,
        }
    }

    pub async fn coinpaprika_get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{}", self.coinpaprika_base, path);
        let mut req = self.http.get(&url);

        if let Some(key) = &self.api_key {
            req = req.header("Authorization", key.as_str());
        }

        if !params.is_empty() {
            req = req.query(params);
        }

        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            match status {
                StatusCode::PAYMENT_REQUIRED => {
                    bail!(
                        "This command requires a CoinPaprika API key (Starter plan or higher).\n\n\
                         Get your API key:  https://coinpaprika.com/api/\n\
                         Set your key:      coinpaprika-cli config set-key <YOUR_KEY>\n\n\
                         Available plans:\n\
                         \x20 Starter    — Historical data, ticker history, changelog\n\
                         \x20 Business   — All Starter features + ID mappings, priority support\n\
                         \x20 Enterprise — Custom limits, dedicated support\n\n\
                         Already have a key? Make sure it's configured:\n\
                         \x20 coinpaprika-cli config show"
                    );
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    bail!("Rate limit exceeded. Wait a moment and try again.");
                }
                StatusCode::FORBIDDEN => {
                    bail!("Invalid API key. Check your key with `coinpaprika-cli config show`");
                }
                StatusCode::NOT_FOUND => {
                    bail!("Not found. Check the ID format (e.g., btc-bitcoin for Bitcoin). API response: {body}");
                }
                s if s.is_server_error() => {
                    bail!("CoinPaprika API is temporarily unavailable. Try again shortly. ({status})");
                }
                _ => {
                    bail!("CoinPaprika API error {status}: {body}");
                }
            }
        }

        Ok(resp.json().await?)
    }
}
