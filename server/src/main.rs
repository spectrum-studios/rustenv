mod controllers;
mod pool;
mod strategies;
mod types;

use std::net::SocketAddr;
use std::path::PathBuf;

use axum::Router;
use controllers::auth_controller;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        match dotenv::dotenv() {
            Ok(path) => path,
            Err(e) => {
                println!("dotenv failed: {:?}", e);
                PathBuf::from("")
            }
        };
    }

    let cors = CorsLayer::permissive()
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .expose_headers(Any);

    let app = Router::new()
        .nest("/auth", auth_controller::routes())
        .layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(e) => panic!("Server failed to start on http://{}: {:?}", addr, e),
    }
}
