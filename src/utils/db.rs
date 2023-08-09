use super::structs::{User, UserLogin};
use dotenv::dotenv;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Pool,
};
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
