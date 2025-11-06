#![allow(unused)] // For beginning only.

use axum::{
    Router, 
    routing::get,
};
use axum::response::{Html, IntoResponse};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn handler_hello() -> impl IntoResponse {
    println!(">>> {:<12} - handler_hello", "HANDLER");

    return Html("Hello <strong>World!!!</strong>");
}

#[tokio::main]
async fn main() {
    // Create a route.
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );

    // Define the address to bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(">>> LISTENING on {addr}\n");

    // Create a TCP listener and bind it to the address.
    let listener = TcpListener::bind(&addr).await.unwrap();

    // Serve the Axum application with the listener.
    axum::serve(listener, routes_hello).await.unwrap();
}
