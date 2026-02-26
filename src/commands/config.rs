use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Show current configuration
    Show,
    /// Set CoinPaprika API key
    #[command(name = "set-key")]
    SetKey {
        /// Your CoinPaprika API key
        key: String,
    },
    /// Reset configuration (delete config file)
    Reset,
}

pub async fn execute(cmd: ConfigCommand) -> Result<()> {
    match cmd {
        ConfigCommand::Show => {
            let key_source = crate::config::key_source(None);
            let api_key = crate::config::resolve_api_key(None);
            let config_path = crate::config::config_path()?;

            crate::output::config::print_config_show(
                &config_path.display().to_string(),
                api_key.as_deref(),
                key_source,
            );
        }
        ConfigCommand::SetKey { key } => {
            crate::config::save_api_key(&key)?;
            println!("API key saved to {}", crate::config::config_path()?.display());
            println!("Key: {}", crate::config::mask_key(&key));
        }
        ConfigCommand::Reset => {
            crate::config::delete_config()?;
            println!("Configuration deleted.");
        }
    }
    Ok(())
}
