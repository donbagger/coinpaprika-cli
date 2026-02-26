use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub currencies: Option<Vec<SearchCurrency>>,
    pub exchanges: Option<Vec<SearchExchange>>,
    pub icos: Option<Vec<serde_json::Value>>,
    pub people: Option<Vec<SearchPerson>>,
    pub tags: Option<Vec<SearchTag>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchCurrency {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub rank: Option<i64>,
    pub is_active: Option<bool>,
    #[serde(rename = "type")]
    pub coin_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchExchange {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchPerson {
    pub id: Option<String>,
    pub name: Option<String>,
    pub teams_count: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchTag {
    pub id: Option<String>,
    pub name: Option<String>,
    pub coin_counter: Option<i64>,
    pub ico_counter: Option<i64>,
}

pub async fn execute(
    client: &ApiClient,
    query: &str,
    categories: Option<&str>,
    limit: usize,
    modifier: Option<&str>,
    output: OutputFormat,
    raw: bool,
) -> Result<()> {
    let limit_str = limit.to_string();
    let mut params: Vec<(&str, &str)> = vec![
        ("q", query),
        ("limit", &limit_str),
    ];
    if let Some(cats) = categories {
        params.push(("categories", cats));
    }
    if let Some(m) = modifier {
        params.push(("modifier", m));
    }

    let result: SearchResult = client.coinpaprika_get("/search", &params).await?;
    match output {
        OutputFormat::Table => crate::output::search::print_search_results(&result),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&result, crate::output::ResponseMeta::coinpaprika("/search"), raw)?;
        }
    }
    Ok(())
}
