use axum::{
    extract::Json,
    http::{HeaderValue, Method},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // logs
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "backend=info,axum=info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // allow your local Dioxus dev server
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://127.0.0.1:8080"),
            HeaderValue::from_static("http://localhost:8080"),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    // routes
    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/api/contact", post(contact_handler))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // bind + serve (Axum 0.7 way)
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct ContactPayload {
    name: String,
    email: String,
    message: String,
}

async fn contact_handler(Json(body): Json<ContactPayload>) -> impl IntoResponse {
    tracing::info!("contact from {} <{}>: {}", body.name, body.email, body.message);
    axum::Json(serde_json::json!({ "status": "received" }))
}
