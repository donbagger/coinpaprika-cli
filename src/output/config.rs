use crate::output::{detail_field, print_detail_table};
use crate::config::mask_key;

pub fn print_config_show(config_path: &str, api_key: Option<&str>, key_source: &str) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "Config File", config_path.to_string());
    detail_field!(rows, "API Key", api_key.map(mask_key).unwrap_or_else(|| "Not set".into()));
    detail_field!(rows, "Key Source", key_source.to_string());
    detail_field!(rows, "CoinPaprika URL", if api_key.is_some() {
        "https://api-pro.coinpaprika.com/v1".to_string()
    } else {
        "https://api.coinpaprika.com/v1".to_string()
    });
    print_detail_table(rows);
}
