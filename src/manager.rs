use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_error_handling=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    loop {
        println!("poop")
    }
}
