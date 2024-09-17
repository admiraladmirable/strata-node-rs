use clap::Parser;
use node::app_config::NodeConfig;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod crypto;
mod node;

fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    init_logger();

    let args = NodeConfig::parse();
    run(&args).await;
}
