use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConvertResult {
    pub base_currency_id: Option<String>,
    pub base_currency_name: Option<String>,
    pub base_price_last_updated: Option<String>,
    pub quote_currency_id: Option<String>,
    pub quote_currency_name: Option<String>,
    pub quote_price_last_updated: Option<String>,
    pub amount: Option<f64>,
    pub price: Option<f64>,
}

pub async fn execute(client: &ApiClient, base_id: &str, quote_id: &str, amount: f64, output: OutputFormat, raw: bool) -> Result<()> {
    let amount_str = amount.to_string();
    let result: ConvertResult = client.coinpaprika_get(
        "/price-converter",
        &[
            ("base_currency_id", base_id),
            ("quote_currency_id", quote_id),
            ("amount", &amount_str),
        ],
    ).await?;
    match output {
        OutputFormat::Table => crate::output::convert::print_convert_result(&result),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&result, crate::output::ResponseMeta::coinpaprika("/convert"), raw)?;
        }
    }
    Ok(())
}
