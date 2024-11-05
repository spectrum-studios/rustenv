use std::net::SocketAddr;

use axum::Router;
use axum::http::Method;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

pub async fn app(port: u16) {
    let app = Router::new().layer(CorsLayer::new().allow_origin(Any).allow_methods(vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::PATCH,
    ]));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Backend listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
