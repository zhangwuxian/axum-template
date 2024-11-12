use clap::Parser;
use log::info;
use tokio::signal;
use tokio::sync::broadcast;

use common::config::init_app_conf_by_path;
use common::log::init_app_log;
use server::http::server::start_http_server;
use server::http::state::HttpServerState;

pub const DEFAULT_APP_CONFIG: &str = "config/http-server.toml";

#[derive(Parser, Debug)]
#[command(author="ztom", version, about, long_about = None)]
#[command(next_line_help = true)]
struct ArgsParams {
    #[arg(short, long, default_value_t=String::from(DEFAULT_APP_CONFIG))]
    conf: String,
}

#[tokio::main]
pub async fn main() {
    let args = ArgsParams::parse();
    // init app config
    init_app_conf_by_path(&args.conf);
    // init logger
    init_app_log();

    let (stop_send, _) = broadcast::channel(2);
    let stop_sx = stop_send.clone();
    tokio::spawn(async move {
        let state = HttpServerState::default();
        start_http_server(state, stop_sx).await;
    });

    awaiting_stop(stop_send.clone()).await;
}

pub async fn awaiting_stop(stop_send: broadcast::Sender<bool>) {
    signal::ctrl_c().await.expect("failed to listen for event");
    match stop_send.send(true) {
        Ok(_) => {
            info!("When ctrl + c is received, the server starts to stop");
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
