use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyInfo {
    pub plan: Option<String>,
    pub usage: Option<serde_json::Value>,
    pub message: Option<String>,
}

pub async fn execute_key_info(client: &ApiClient, output: OutputFormat, raw: bool) -> Result<()> {
    let info: KeyInfo = client.coinpaprika_get("/key/info", &[]).await?;
    match output {
        OutputFormat::Table => crate::output::api_management::print_key_info(&info),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&info, crate::output::ResponseMeta::coinpaprika("/key/info"), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_mappings(client: &ApiClient, output: OutputFormat, raw: bool) -> Result<()> {
    let mappings: serde_json::Value = client.coinpaprika_get("/coins/mappings", &[]).await?;
    match output {
        OutputFormat::Table => {
            println!("Coin ID Mappings:");
            println!("{}", serde_json::to_string_pretty(&mappings)?);
            crate::output::print_coinpaprika_footer();
        }
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&mappings, crate::output::ResponseMeta::coinpaprika("/coins/mappings"), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_changelog(client: &ApiClient, limit: usize, page: usize, output: OutputFormat, raw: bool) -> Result<()> {
    let limit_str = limit.to_string();
    let page_str = page.to_string();
    let changelog: serde_json::Value = client.coinpaprika_get(
        "/changelog/ids",
        &[("limit", &limit_str), ("page", &page_str)],
    ).await?;
    match output {
        OutputFormat::Table => {
            println!("Changelog:");
            println!("{}", serde_json::to_string_pretty(&changelog)?);
            crate::output::print_coinpaprika_footer();
        }
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&changelog, crate::output::ResponseMeta::coinpaprika("/changelog"), raw)?;
        }
    }
    Ok(())
}
