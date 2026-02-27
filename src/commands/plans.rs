use anyhow::Result;

pub fn execute() -> Result<()> {
    println!();
    println!("  ─── Free tier ($0/mo, no API key needed) ───");
    println!();
    println!("  Rate limits");
    println!("    20,000 calls/month");
    println!("    Data updates every ~10 minutes");
    println!();
    println!("  Coverage");
    println!("    25+ endpoints");
    println!("    2,000 assets");
    println!("    Personal use");
    println!();
    println!("  Historical data");
    println!("    Daily OHLCV:    up to 1 year back");
    println!("    Hourly OHLCV:   last 24 hours");
    println!("    OHLCV interval: 24h only");
    println!("    5-min / ticker history: not available");
    println!();
    println!("  Not included");
    println!("    Circulating supply");
    println!("    API ID mappings");
    println!("    WebSockets");
    println!("    Redistribution rights");
    println!("    SLA / dedicated infrastructure");
    println!();
    println!("  ─── Need more? ───");
    println!();
    println!("  Paid plans add: full history, 5-min intervals, circulating supply,");
    println!("  higher limits, WebSockets, commercial use, and priority support.");
    println!();
    println!("  See current pricing:  https://coinpaprika.com/api/pricing");
    println!("  Set your API key:     coinpaprika-cli config set-key <KEY>");
    println!();

    Ok(())
}
