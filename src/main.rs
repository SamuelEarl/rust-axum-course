#![allow(unused)] // For beginning only.

use axum::{
    Router, 
    extract::{
        Path,
        Query,
    }, 
    http::StatusCode, 
    response::{
        Html,
        IntoResponse,
    }, 
    routing::{
        MethodRouter, 
        get, 
        get_service
    }
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Create a route.
    let app = Router::new()
        .merge(app_router())
        .fallback_service(static_routes());

    // Define the address to bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(">>> LISTENING on {addr}\n");

    // Create a TCP listener and bind it to the address.
    let listener = TcpListener::bind(&addr).await.unwrap();

    // Serve the Axum application with the listener.
    axum::serve(listener, app).await.unwrap();
}

fn app_router() -> Router {
    return Router::new()
        .route(
            "/query-params-route",
            get(query_params_handler),
        )
        .route(
            "/path-params-route/{name}",
            get(path_params_handler),
        );
}

fn static_routes() -> MethodRouter {
    async fn handle_404() -> (StatusCode, &'static str) {
        return (StatusCode::NOT_FOUND, "Resource not found.");
    }

    return get_service(ServeDir::new("/").not_found_service(handle_404.into_service()));
}

// Traits are used to define shared behaviors — like printing, comparing, cloning, or serializing.
// The `#[derive(...)]` macro auto-implements traits for the data structure that follows this macro.
#[derive(Debug, Deserialize)]
struct QueryParamsStruct {
    name: Option<String>,
}

// You can destructure query params by passing `Query(params)` (instead of simply `params` or `{ params }` as in JavaScript) to the route handler function.
// Then you can reference the params in your function like this: `params.name`.
async fn query_params_handler(Query(params): Query<QueryParamsStruct>) -> impl IntoResponse {
    // The `Debug` trait allows you to use the `{:?}` formatter to print the internal state of a struct or enum.
    // Without the `Debug` trait, the following line wouldn’t compile.
    println!(">>> {:<12} - query_params_handler - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    return Html(format!("Query Params: <strong>{name}</strong>"));
}

async fn path_params_handler(Path(name): Path<String>) -> impl IntoResponse {
    println!(">>> {:<12} - path_params_handler - {name:?}", "HANDLER");

    return Html(format!("Path Params: <strong>{name}</strong>"));
}
