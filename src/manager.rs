use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::ManagerConfig;

pub struct FileData {
    hash: String,
    original_name: String,
    // owner: StorageNode,
}

struct AppState {
    // database connection
    // config
    //
}

pub async fn main(config: ManagerConfig) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_error_handling=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("running FloppaScale Manager");
}
