/* Event management application with ability to have unique users through sign up and login
    and ability to create events and invite other users to the event with specific date for the event
    as well as dashboard for each user to see the events they are invited to and the events they created
    and the ability to accept or decline the invitation to the event
    and the ability to delete the event they created
    and the ability to see the users that accepted the invitation to the event they created
    and the ability to see the users that declined the invitation to the event they created
    and the users are mapped via a unique id that link users table to events table
*/
use axum::{
    extract::{rejection::JsonRejection, Json},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
struct User {
    user: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", post(login))
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

async fn login(payload: Result<Json<Value>, JsonRejection>) -> Result<String, ()> {
    if let Ok(payload) = payload {
        let value = json!(*payload);
        println!("user: {}, id {}", value["user"], value["id"]);
        println!("value: {:?}", value);
        let user: User = serde_json::from_value(value).unwrap();
        println!("user: {}", user.user);
    }
    Ok("User logged in".to_string())
}
