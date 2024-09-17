use clap::{command, Parser};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
pub struct NodeConfig {
    #[arg(long, default_value = "/tmp/strata/data", env = "DATA_DIR")]
    pub data_dir: String,

    #[arg(long, default_value = "/tmp/strata/staking", env = "STAKING_DIR")]
    pub staking_dir: String,

    #[arg(long, default_value = "localhost", env = "RPC_BIND_HOST")]
    pub rpc_bind_host: String,

    #[arg(long, default_value_t = 9084, env = "RPC_BIND_PORT")]
    pub rpc_bind_port: u16,

    #[arg(long, default_value = "localhost", env = "P2P_BIND_HOST")]
    pub p2p_bind_host: String,

    #[arg(long, default_value_t = 9085, env = "P2P_BIND_PORT")]
    pub p2p_bind_port: u16,
}
