use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use tabled::settings::object::Columns;
use tabled::settings::{Modify, Style, Width};
use tabled::Table;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

// --- Attribution / _meta wrapper ---

#[derive(Serialize)]
pub struct WrappedResponse<'a, T: Serialize> {
    pub data: &'a T,
    pub _meta: ResponseMeta,
}

#[derive(Serialize)]
pub struct ResponseMeta {
    pub source: String,
    pub url: String,
    pub api_docs: String,
    pub attribution: String,
    pub timestamp: String,
}

impl ResponseMeta {
    pub fn coinpaprika(entity_path: &str) -> Self {
        Self {
            source: "CoinPaprika".into(),
            url: format!("https://coinpaprika.com{entity_path}"),
            api_docs: "https://api.coinpaprika.com".into(),
            attribution: "Powered by CoinPaprika · Free crypto market data".into(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

// --- Shared output helpers ---

pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut truncated: String = s.chars().take(max.saturating_sub(1)).collect();
    truncated.push('\u{2026}');
    truncated
}

pub fn truncate_address(addr: &str) -> String {
    if addr.len() <= 13 {
        return addr.to_string();
    }
    format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
}

pub fn format_usd(n: f64) -> String {
    if n.abs() >= 1_000_000_000_000.0 {
        format!("${:.1}T", n / 1_000_000_000_000.0)
    } else if n.abs() >= 1_000_000_000.0 {
        format!("${:.1}B", n / 1_000_000_000.0)
    } else if n.abs() >= 1_000_000.0 {
        format!("${:.1}M", n / 1_000_000.0)
    } else if n.abs() >= 1_000.0 {
        format!("${:.1}K", n / 1_000.0)
    } else {
        format!("${n:.2}")
    }
}

pub fn format_price(n: f64) -> String {
    if n >= 1.0 {
        format!("${n:.2}")
    } else if n >= 0.01 {
        format!("${n:.4}")
    } else {
        format!("${n:.8}")
    }
}

pub fn format_percent(n: f64) -> String {
    if n >= 0.0 {
        format!("+{n:.2}%")
    } else {
        format!("{n:.2}%")
    }
}

pub fn format_supply(n: f64) -> String {
    if n >= 1_000_000_000.0 {
        format!("{:.1}B", n / 1_000_000_000.0)
    } else if n >= 1_000_000.0 {
        format!("{:.1}M", n / 1_000_000.0)
    } else if n >= 1_000.0 {
        format!("{:.1}K", n / 1_000.0)
    } else {
        format!("{n:.0}")
    }
}

#[allow(dead_code)]
pub fn print_json<T: Serialize>(data: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(data)?);
    Ok(())
}

pub fn print_json_wrapped<T: Serialize>(data: &T, meta: ResponseMeta, raw: bool) -> Result<()> {
    if raw {
        println!("{}", serde_json::to_string_pretty(data)?);
    } else {
        let wrapped = WrappedResponse { data, _meta: meta };
        println!("{}", serde_json::to_string_pretty(&wrapped)?);
    }
    Ok(())
}

pub fn print_detail_table(rows: Vec<[String; 2]>) {
    let table = Table::from_iter(rows)
        .with(Style::rounded())
        .with(Modify::new(Columns::first()).with(Width::wrap(20)))
        .with(Modify::new(Columns::last()).with(Width::wrap(80)))
        .to_string();
    println!("{table}");
}

pub fn print_coinpaprika_footer() {
    println!(
        "\n Data: CoinPaprika (coinpaprika.com) \u{00b7} Free API: api.coinpaprika.com"
    );
}

macro_rules! detail_field {
    ($rows:expr, $label:expr, $val:expr) => {
        $rows.push([$label.into(), $val]);
    };
}

pub(crate) use detail_field;

pub mod global;
pub mod coins;
pub mod tickers;
pub mod ohlcv;
pub mod exchanges;
pub mod tags;
pub mod people;
pub mod search;
pub mod convert;
pub mod contracts;
pub mod api_management;
pub mod config;
pub mod status;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_long_string_appends_ellipsis() {
        let result = truncate("hello world", 5);
        assert_eq!(result.chars().count(), 5);
        assert!(result.ends_with('\u{2026}'));
    }

    #[test]
    fn format_usd_billions() {
        assert_eq!(format_usd(1_500_000_000.0), "$1.5B");
    }

    #[test]
    fn format_usd_millions() {
        assert_eq!(format_usd(12_400_000.0), "$12.4M");
    }

    #[test]
    fn format_usd_thousands() {
        assert_eq!(format_usd(5_200.0), "$5.2K");
    }

    #[test]
    fn format_usd_small() {
        assert_eq!(format_usd(42.5), "$42.50");
    }

    #[test]
    fn format_percent_positive() {
        assert_eq!(format_percent(2.34), "+2.34%");
    }

    #[test]
    fn format_percent_negative() {
        assert_eq!(format_percent(-1.23), "-1.23%");
    }

    #[test]
    fn format_price_large() {
        assert_eq!(format_price(97234.56), "$97234.56");
    }

    #[test]
    fn format_price_small() {
        assert_eq!(format_price(0.001234), "$0.00123400");
    }

    #[test]
    fn truncate_address_short() {
        assert_eq!(truncate_address("0x1234567890abcdef"), "0x1234...cdef");
    }
}
