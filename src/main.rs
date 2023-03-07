use axum::{
    Json,
    Router,
    Server, routing::get,
    extract::State,
    response::IntoResponse,
    response::Response,
    response::Html,
};
use sysinfo::{System, SystemExt, CpuExt};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.mjs", get(indexjs_get))
        .route("/api/cpus", get(cpus_get)).with_state(AppState {
        sys: Arc::new(Mutex::new(System::new())),
    });
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());

    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();

    Html(markup)
}

#[axum::debug_handler]
async fn indexjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.mjs").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    Json(v)
}
