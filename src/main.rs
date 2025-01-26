mod handlers;
mod safener;

use crate::handlers::handlers::{change_state_handler, get_state_handler};
use crate::safener::safener::Safener;
use axum::Router;
use axum::routing::get;
use dotenv::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main] // This attribute allows async main function
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8999".to_string());
    let safe_path = std::env::var("SAFE_PATH").unwrap_or_else(|_| "/tmp".to_string());
    tracing_subscriber::fmt().init();

    dotenv().ok();
    tracing::info!("router initialized, now listening on port {}", port);

    let sentinel = Safener::new(safe_path);
    let sentinel_state = Arc::new(sentinel);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    let app = Router::new()
        .route("/state", get(get_state_handler))
        .route("/set_state", get(change_state_handler))
        .with_state(Arc::clone(&sentinel_state));

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}
