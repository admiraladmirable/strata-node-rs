use tracing::info;

use super::app_config::NodeConfig;

pub struct NodeApp;

impl NodeApp {
    pub async fn run(args: &NodeConfig) {
        info!("Launching node with args={args:?}");
    }
}
