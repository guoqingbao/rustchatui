use anyhow::Result;
use axum::{Router, response::IntoResponse};
use include_dir::{Dir, include_dir};
use tower_http::cors::{Any, CorsLayer};

static DIST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dist");

pub async fn start_ui_server(port: u16) -> Result<()> {
    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Custom handler to serve files from DIST_DIR
    let app = Router::new()
        .fallback(axum::routing::get(serve_file))
        .layer(cors);

    let addr = format!("0.0.0.0:{port}");
    println!("🖥️ Rust Chat UI server running at http://localhost:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn serve_file(req: axum::http::Request<axum::body::Body>) -> impl IntoResponse {
    let path = req.uri().path().trim_start_matches('/');

    let file = if path.is_empty() {
        DIST_DIR.get_file("index.html")
    } else {
        DIST_DIR.get_file(path)
    };

    match file {
        Some(f) => {
            let mime = mime_guess::from_path(f.path()).first_or_octet_stream();
            (
                [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                f.contents(),
            )
                .into_response()
        }
        None => (axum::http::StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}
