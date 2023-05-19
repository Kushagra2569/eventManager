/* Event management applicaation with ability to have unique users through sign up and login
    and ability to create events and invite other users to the event with specific date for the event
    as well as dashboard for each user to see the events they are invited to and the events they created
    and the ability to accept or decline the invitation to the event
    and the ability to delete the event they created
    and the ability to see the users that accepted the invitation to the event they created
    and the ability to see the users that declined the invitation to the event they created
    and the users are mapped via a unique id that link users table to events table
*/
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", get(login))
        .fallback(fallback_handler);

    axum::Server::bind(&"127.0.0.1:3042".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback_handler(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

async fn login() -> Result<String, ()> {
    Ok("logged in".to_string())
}
