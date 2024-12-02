mod config;
mod manager;
mod storage;
use config::ManagerConfig;
use std::env;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subcommand = env::args().nth(1).unwrap_or_else(|| "".to_string());

    tokio::select! {
        _ = async {
            match subcommand.as_str() {
                "storage" => storage::main().await,
                "manager" => manager::main(ManagerConfig::load(env::args().nth(2).unwrap_or_else(|| "".to_string())).await.unwrap()).await,
                _ => println!("error!"),
            }
        } => {}

        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
        }
    }

    Ok(())
}
