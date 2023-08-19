use super::structs::{Event, User, UserLogin};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Row};
use std::env;

pub async fn connect_db() -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("POSTGRES_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;

    Ok(pool)
}

pub async fn insert_user(
    conn: Pool<sqlx::Postgres>,
    user_login: &UserLogin,
) -> Result<String, String> {
    let query1 = sqlx::query(
        "INSERT INTO userlogin (username, password, email, id) VALUES ($1, $2, $3, $4)",
    )
    .bind(&user_login.username)
    .bind(&user_login.password)
    .bind(&user_login.email)
    .bind(&user_login.id)
    .execute(&conn)
    .await;

    if query1.is_ok() {
        let query2 = sqlx::query("INSERT INTO users (id, full_name, role) VALUES ($1, $2, 'user')")
            .bind(&user_login.id)
            .bind(&user_login.username)
            .execute(&conn)
            .await;

        if query2.is_ok() {
            return Ok(user_login.id.to_string());
        } else {
            let _ = sqlx::query("DELETE FROM userlogin WHERE id = $1")
                .bind(&user_login.id)
                .execute(&conn)
                .await;

            println!("{:?}", query2);

            return Err("Error inserting user into user table".to_string());
        }
    } else {
        println!("{:?}", query1);
        return Err("Error inserting user into userlogin table".to_string());
    }
}

pub async fn login_user(
    conn: Pool<sqlx::Postgres>,
    user_login: &UserLogin,
) -> Result<String, String> {
    let query1 = sqlx::query("SELECT * FROM userlogin WHERE username = $1")
        .bind(&user_login.username)
        .fetch_one(&conn)
        .await;

    if query1.is_ok() {
        let row = query1.unwrap();
        let pass = row.get::<String, &str>("password");
        let userid = row.get::<String, &str>("id");
        println!("id: {}", userid);
        if pass == user_login.password {
            return Ok("login successful".to_string());
        } else {
            return Err("password incorrect".to_string());
        }
    } else {
        return Err("username not found".to_string());
    }

    //TODO check for query error if user is not present
    //TODO return user object mapped from userlogin and user tables
}

pub async fn create_event(conn: Pool<sqlx::Postgres>, event: &Event) -> Result<String, String> {
    let query1 = sqlx::query(
        "INSERT INTO events (event_id, event_name, event_date, owner_id) VALUES ($1, $2, $3, $4)",
    )
    .bind(&event.id)
    .bind(&event.name)
    .bind(&event.date)
    .bind(&event.owner_id)
    .execute(&conn)
    .await;

    if query1.is_err() {
        println!("{:?}", query1);
        return Err("Error inserting event into events table".to_string());
    }

    if query1.is_ok() {
        let query2 = sqlx::query("INSERT INTO event_users (event_id, user_id) VALUES ($1, $2)")
            .bind(&event.id)
            .bind(&event.owner_id)
            .execute(&conn)
            .await;

        if query2.is_ok() {
            return Ok("ok".to_string());
        } else {
            let _ = sqlx::query("DELETE FROM events WHERE event_id = $1")
                .bind(&event.id)
                .execute(&conn)
                .await;
            return Err("Error inserting event into event_users table".to_string());
        }
    }

    Ok("ok".to_string())
}
