use anyhow::Result;
use axum::{response::IntoResponse, Router};
use colored::*;
use include_dir::{include_dir, Dir};
use inquire::Select;
use local_ip_address::local_ip;
use tower_http::cors::{Any, CorsLayer};
static DIST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dist");

#[derive(serde::Serialize)]
#[allow(non_snake_case)]
struct AppConfig {
    serverUrl: String,
    apiKey: String,
}

pub fn choose_url(port: u16) -> (String, bool) {
    let ip = local_ip().unwrap_or("127.0.0.1".parse().unwrap());

    let local_url = format!("http://localhost:{port}/v1/");
    let lan_url = format!("http://{ip}:{port}/v1/");

    let lan_option = format!("LAN Access this API server   → {}", lan_url);
    let local_option = format!("Local Access this API server → {}", local_url);

    let options = vec![lan_option.clone(), local_option];

    let ans = Select::new(
        "Choose how the Chat UI connects to the API server:",
        options,
    )
    .with_help_message("Use ↑ / ↓ to navigate, press Enter to confirm.")
    .prompt()
    .unwrap_or_else(|_| lan_option); // fallback to LAN if prompt fails (NotTTY)

    if ans.contains("LAN Access") {
        (lan_url, false)
    } else {
        (local_url, true)
    }
}

pub async fn start_ui_server(
    ui_port: u16,
    api_port: Option<u16>,
    server_url: Option<String>,
    api_key: Option<String>,
) -> Result<()> {
    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let (api_url, is_local) = if api_port.is_some() {
        choose_url(api_port.unwrap())
    } else {
        (server_url.unwrap(), false)
    };

    let api_key = if api_key.is_some() {
        api_key.unwrap()
    } else {
        "".to_string()
    };

    // Custom handler to serve files from DIST_DIR
    let app = Router::new()
        .route(
            "/app-config.json", // Capture the API port to return in the config
            axum::routing::get(move || async move {
                axum::Json(AppConfig {
                    serverUrl: api_url,
                    apiKey: api_key,
                })
            }),
        )
        .fallback(axum::routing::get(serve_file))
        .layer(cors);

    let addr = format!("0.0.0.0:{ui_port}");

    if is_local {
        println!(
            "\n🖥️ Chat UI server running at ({}): {} (Local Access Only)",
            format!("click to open").bold(),
            format!("http://localhost:{ui_port}").blue().bold()
        );
    } else {
        let ip = local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
        println!(
            "\n🖥️ Chat UI server running at ({}): {} (Remote Access, {})",
            format!("click to open").bold(),
            format!("http://{ip}:{ui_port}").blue().bold(),
            format!("Recommended").red().bold()
        );
    }

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
