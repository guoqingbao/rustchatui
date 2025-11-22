use anyhow::Result;
use axum::Router;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

pub async fn start_ui_server(port: u16, dist_path: impl Into<String>) -> Result<()> {
    let dist = dist_path.into();
    let full_dist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(dist);

    let cors = CorsLayer::new()
        .allow_origin(Any) // same as "*"
        .allow_methods(Any)
        .allow_headers(Any);

    // Serve the dist folder
    let app = Router::new()
        .nest_service("/", ServeDir::new(full_dist_path))
        .layer(cors);

    let addr = format!("localhost:{port}");
    println!("\r\n🚀 Rust Chat UI server running at (click to open) http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
