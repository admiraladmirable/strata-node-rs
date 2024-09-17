use tracing::{info, Level};

use super::app_config::NodeConfig;

async fn run(args: &NodeConfig) {
    info!("Launching node with args={args:?}");
}
