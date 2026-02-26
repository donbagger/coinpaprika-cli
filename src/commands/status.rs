use anyhow::Result;
use serde::Serialize;
use std::time::Instant;

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Serialize)]
pub struct StatusResult {
    pub coinpaprika: ApiStatus,
    pub api_key_configured: bool,
}

#[derive(Debug, Serialize)]
pub struct ApiStatus {
    pub status: String,
    pub response_time_ms: u128,
}

pub async fn execute(client: &ApiClient, output: OutputFormat, raw: bool) -> Result<()> {
    let cp_start = Instant::now();
    let cp_result: Result<serde_json::Value> = client.coinpaprika_get("/global", &[]).await;
    let cp_time = cp_start.elapsed().as_millis();

    let api_key_configured = crate::config::resolve_api_key(None).is_some();

    let result = StatusResult {
        coinpaprika: ApiStatus {
            status: if cp_result.is_ok() { "OK".into() } else { "ERROR".into() },
            response_time_ms: cp_time,
        },
        api_key_configured,
    };

    match output {
        OutputFormat::Table => crate::output::status::print_status(&result),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&result, crate::output::ResponseMeta::coinpaprika("/status"), raw)?;
        }
    }
    Ok(())
}
