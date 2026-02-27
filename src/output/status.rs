use crate::commands::status::StatusResult;
use crate::output::{detail_field, print_detail_table};

pub fn print_status(result: &StatusResult) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "CoinPaprika API", format!("{} ({}ms)", result.coinpaprika.status, result.coinpaprika.response_time_ms));
    detail_field!(rows, "API Key", if result.api_key_configured { "Configured".to_string() } else { "Not set (free tier)".to_string() });
    detail_field!(rows, "Plan", if result.api_key_configured { "Paid (run key-info for details)".to_string() } else { "Free (20,000 calls/mo)".to_string() });
    print_detail_table(rows);
}
