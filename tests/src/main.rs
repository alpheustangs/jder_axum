mod router;

use std::net::SocketAddr;

use axum::{
    Router, extract::connect_info::IntoMakeServiceWithConnectInfo, serve,
};
use tokio::net::TcpListener;

use crate::router::create_router;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let router: IntoMakeServiceWithConnectInfo<Router, SocketAddr> =
        create_router();

    // address
    let addr: String = "0.0.0.0:4001".to_string();

    // listener
    let listener: TcpListener = match TcpListener::bind(&addr).await {
        | Ok(_listener) => _listener,
        | Err(e) => panic!("Unable to bind address {}: {}", addr, e),
    };

    // serve
    println!("Server running on {}", addr);

    if let Err(e) = serve(listener, router).await {
        panic!("Unable to start server: {}", e)
    };
}
