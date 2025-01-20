use axum::{
    routing::get,
    Router,
    Json,
    http::\{HeaderValue, Method},
};
use tower_http::cors::{CorsLayer, Any};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn hello() -> Json<Message> {
    Json(Message {
        text: "Hello from Rust backend!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/hello", get(hello))
        .layer(cors);

    println!("Backend server running on http://localhost:3001");
    
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
