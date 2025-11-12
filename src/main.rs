use bytes::Bytes;
use futures_util::StreamExt;

use axum::{extract::BodyStream, routing::get, routing::post, Router};
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {

    // build service routes
    let app = Router::new()
        .route("/", get(probe))
        .route("/service", post(service));

    // run service on port 8080
    let addr = "0.0.0.0:8080";
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::Server::from_tcp(tcp_listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// return OK with body to pass readiness probe
async fn probe() -> &'static str {
    "Rust service is alive!"
}

// echo body data in a separate post handler
async fn service(mut stream: BodyStream) -> Bytes {
    if let Some(Ok(s)) = stream.next().await {
        s
    } else {
        Bytes::new()
    }
}