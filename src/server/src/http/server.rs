use crate::http::auth::auth_middleware;
use crate::http::handlers::{get_machine_list_handler, login_handler};
use crate::http::state::HttpServerState;
use axum::routing::post;
use axum::Router;
use common::config::get_app_conf;
use log::info;
use std::net::SocketAddr;
use tokio::select;
use tokio::sync::broadcast;

pub async fn start_http_server(state: HttpServerState, stop_sx: broadcast::Sender<bool>) {
    let config = get_app_conf();
    let ip: SocketAddr = match format!("0.0.0.0:{}", config.server.port).parse() {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let app = routes(state);

    let mut stop_rx = stop_sx.subscribe();

    let listener = match tokio::net::TcpListener::bind(ip).await {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e);
        }
    };

    info!(
        "HTTP Server started. Listening on port:{}",
        config.server.port
    );

    select! {
        val = stop_rx.recv() => {
            if let Ok(flag) = val {
                if flag {
                    info!("HTTP Server stopped successfully");
                }
            }
        },
        val = axum::serve(listener, app.clone()) => {
            match val{
                Ok(()) => {},
                Err(e) => {
                    panic!("{}",e);
                }
            }
        }
    }
}

fn routes(state: HttpServerState) -> Router {
    let no_auth_routers = Router::new().route("/login", post(login_handler));

    let auth_routers = Router::new()
        .route("/get", post(get_machine_list_handler))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = Router::new().merge(no_auth_routers).merge(auth_routers);
    app.with_state(state)
}
