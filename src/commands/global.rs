use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalData {
    pub market_cap_usd: Option<f64>,
    pub volume_24h_usd: Option<f64>,
    pub bitcoin_dominance_percentage: Option<f64>,
    pub cryptocurrencies_number: Option<i64>,
    pub market_cap_ath_value: Option<f64>,
    pub market_cap_ath_date: Option<String>,
    pub volume_24h_ath_value: Option<f64>,
    pub volume_24h_ath_date: Option<String>,
    pub volume_24h_percent_from_ath: Option<f64>,
    pub volume_24h_percent_to_ath: Option<f64>,
    pub market_cap_change_24h: Option<f64>,
    pub volume_24h_change_24h: Option<f64>,
    pub last_updated: Option<i64>,
}

pub async fn execute(client: &ApiClient, output: OutputFormat, raw: bool) -> Result<()> {
    let data: GlobalData = client.coinpaprika_get("/global", &[]).await?;
    match output {
        OutputFormat::Table => crate::output::global::print_global(&data),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&data, crate::output::ResponseMeta::coinpaprika(""), raw)?;
        }
    }
    Ok(())
}
