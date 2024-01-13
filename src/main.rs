/* Event management application with ability to have unique users through sign up and login and ability to create events
    and invite other users to the event with specific date for the event
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
use serde_json::{json, Value};

mod utils;
use chrono::prelude::*;
use utils::{
    db,
    structs::{Event, User, UserLogin},
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", post(login))
        .route("/signup", post(sign_up))
        .route("/create_event", post(create_event))
        .route("/join_event", post(join_event))
        .route("/leave_event", post(leave_event))
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

// Example login Payload
// {
//    "username": "username",
//    "password": "password"
//  }
//TODO return user struct from the api
async fn login(payload: Result<Json<Value>, JsonRejection>) -> Result<String, String> {
    if let Ok(payload) = payload {
        let user_data: UserLogin;
        let user: User;
        let mut value = json!(*payload);
        //TODO check if the value is an object and return a proper error
        if !value.is_object() {
            return Err("Invalid JSON".to_string());
        }
        let value_obj = value.as_object_mut().unwrap();
        if !value_obj.contains_key("username") || !value_obj.contains_key("password") {
            return Err("Invalid JSON".to_string());
        }
        user_data = UserLogin {
            username: value_obj
                .get("username")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            password: value_obj
                .get("password")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            id: "0".to_string(),
            email: "not Applicable".to_string(),
        };
        //TODO using get method above even though already checked for key existence
        let connection = db::connect_db().await.unwrap();
        println!("Connected to db");
        let result = db::login_user(connection, &user_data).await;

        if result.is_err() {
            return Err("Invalid credentials".to_string());
        }
        if result.is_ok() {
            user = result.unwrap();
            println!("fullname {}", user.fullname);
            return Ok("User logged in".to_string());
        }
    }

    Ok("User logged in".to_string())
}

//Example sign up Payload
// {
//   "username": "username",
//   "email": "email",
//   "password": "password"
//   }
async fn sign_up(payload: Result<Json<Value>, JsonRejection>) -> Result<String, String> {
    if let Ok(payload) = payload {
        let user_data: UserLogin;
        let mut value = json!(*payload);
        //TODO check if the value is an object and return a proper error
        if !value.is_object() {
            return Err("Invalid payload".to_string());
        }
        let value_obj = value.as_object_mut().unwrap();
        if (!value_obj.contains_key("username") || !value_obj.contains_key("email"))
            || !value_obj.contains_key("password")
        {
            return Err("Invalid payload".to_string());
        }
        user_data = UserLogin {
            username: value_obj
                .get("username")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            email: value_obj
                .get("email")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            password: value_obj
                .get("password")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            id: Uuid::new_v4().as_simple().to_string(),
        };
        //TODO using get method above even though already checked for key existence

        println!("User: {} email: {}", user_data.username, user_data.email);
        let connection = db::connect_db().await.unwrap();
        println!("Connected to db");
        let result = db::insert_user(connection, &user_data).await;

        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err("Error inserting user".to_string());
        }
    }

    Ok("User Signed up".to_string())
}

//Example create event payload
// {
//  "name": "event name",
//  "owner_id": "user id"
//  }
async fn create_event(payload: Result<Json<Value>, JsonRejection>) -> Result<String, String> {
    if let Ok(payload) = payload {
        let event_data: Event;
        let mut value = json!(*payload);
        //TODO check if the value is an object and return a proper error
        if !value.is_object() {
            return Err("Payload is not an object".to_string());
        }
        let value_obj = value.as_object_mut().unwrap();
        if !value_obj.contains_key("name") || !value_obj.contains_key("owner_id") {
            return Err("Payload does not contain all required fields".to_string());
        }
        event_data = Event {
            name: value_obj.get("name").unwrap().as_str().unwrap().to_string(),
            date: Utc::now(),
            id: Uuid::new_v4().as_simple().to_string(),
            owner_id: value_obj
                .get("owner_id")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        };
        //TODO using get method above even though already checked for key existence
        println!("name: {} date: {}", event_data.name, event_data.date);
        let connection = db::connect_db().await.unwrap();
        let result = db::create_event(connection, &event_data).await;

        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err("Error Creating Event".to_string());
        }
    }
    Ok("Event created".to_string())
}

//Example join event Payload
//{ "event_id": "event id", "user_id": "user id"}
async fn join_event(payload: Result<Json<Value>, JsonRejection>) -> Result<String, String> {
    if let Ok(payload) = payload {
        let event_id: String;
        let user_id: String;
        let mut value = json!(*payload);
        //TODO check if the value is an object and return a proper error
        if !value.is_object() {
            return Err("Payload is not an object".to_string());
        }
        let value_obj = value.as_object_mut().unwrap();
        if !value_obj.contains_key("event_id") || !value_obj.contains_key("user_id") {
            return Err("Payload does not contain all required fields".to_string());
        }
        event_id = value_obj
            .get("event_id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        user_id = value_obj
            .get("user_id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        //TODO using get method above even though already checked for key existence
        println!("event_id: {} user_id: {}", event_id, user_id);
        let connection = db::connect_db().await.unwrap();
        let result = db::join_event(connection, &event_id, &user_id).await;

        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err("Error Joining Event".to_string());
        }
    }
    Ok("Event joined".to_string())
}

//Example leave event payload
//{ "event_id": "event id", "user_id": "user id"}
async fn leave_event(payload: Result<Json<Value>, JsonRejection>) -> Result<String, String> {
    if let Ok(payload) = payload {
        let event_id: String;
        let user_id: String;
        let mut value = json!(*payload);
        //TODO check if the value is an object and return a proper error
        if !value.is_object() {
            return Err("Payload is not an object".to_string());
        }
        let value_obj = value.as_object_mut().unwrap();
        if !value_obj.contains_key("event_id") || !value_obj.contains_key("user_id") {
            return Err("Payload does not contain all required fields".to_string());
        }
        event_id = value_obj
            .get("event_id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        user_id = value_obj
            .get("user_id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        //TODO using get method above even though already checked for key existence
        println!("event_id: {} user_id: {}", event_id, user_id);
        let connection = db::connect_db().await.unwrap();
        let result = db::leave_event(connection, &event_id, &user_id).await;

        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err("Error Leaving Event".to_string());
        }
    }
    Ok("Event Left".to_string())
}
