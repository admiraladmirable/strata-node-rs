use clap::Parser;
use env_logger::{Env};
use log::{error, info, warn};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct BifrostArgs {
    #[arg(long, default_value = "/tmp/bifrost/data", env = "DATA_DIR")]
    data_dir: String,

    #[arg(long, default_value = "/tmp/bifrost/staking", env = "STAKING_DIR")]
    staking_dir: String,

    #[arg(long, default_value = "localhost", env = "RPC_BIND_HOST")]
    rpc_bind_host: String,

    #[arg(long, default_value_t = 9084, env = "RPC_BIND_PORT")]
    rpc_bind_port: u16,

    #[arg(long, default_value = "localhost", env = "P2P_BIND_HOST")]
    p2p_bind_host: String,

    #[arg(long, default_value_t = 9085, env = "P2P_BIND_PORT")]
    p2p_bind_port: u16,
}

async fn run(args: &BifrostArgs) {
    info!("Launching node with args={args:?}")
}

fn init_logger() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "trace")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

#[tokio::main]
async fn main() {
    init_logger();

    let args = BifrostArgs::parse();
    run(&args).await;
}
