use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};
 
use serde::{Deserialize, Serialize};
use serde_json::json;

use chrono::NaiveDate;

use sqlx::{postgres::PgPoolOptions, PgPool};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
  println!("Hello, world!");
  //expose env variables
  dotenvy::dotenv().expect("Unable to access .env file");
  //Set variables from the environment variables
  let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the env file");
  //create database pool
  let dp_pool = PgPoolOptions::new()
    .max_connections(16)
    .connect(&database_url)
    .await
    .expect("Can't connect to database");
  //create tcp listener
  let listener = TcpListener::bind(server_address)
    .await
    .expect("Could not create TCP Listener");
  println!("Listening on {}", listener.local_addr().unwrap());
  //compose routes
  let app = Router::new().route("/", get(|| async { "Hello World"}))
                    .route("/people", get(get_people).post(create_person))
                    .route("/people/:person_id", patch(update_person).delete(delete_person))
                    .with_state(dp_pool);
  //serve the app
  axum::serve(listener, app)
  .await
  .expect("Error serving application");
}

#[derive(Serialize)]
struct Person {
    person_id: i32,               // Maps to `integer` type in PostgreSQL
    first_name: Option<String>,           // Maps to `character varying(255)` in PostgreSQL
    last_name: Option<String>,            // Maps to `character varying(255)` in PostgreSQL
    birth_date: Option<chrono::NaiveDate>, // Maps to `date` in PostgreSQL
    is_alive: bool,               // Maps to `boolean` in PostgreSQL
    current_sex: Option<String>,  // Maps to `character varying(50)` in PostgreSQL
    current_alias: Option<String>,// Maps to `character varying(255)` in PostgreSQL
}
async fn get_people(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(Person, "SELECT * FROM people ORDER BY person_id")
    .fetch_all(&pg_pool)
    .await
    .map_err(|e|{
        (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({ "success": false, "message": e.to_string()}).to_string()
        )
    })?;
    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": rows }).to_string()
    ))
}
#[derive(Deserialize)]

struct CreatePersonReq {
    first_name: Option<String>,           // Maps to `character varying(255)` in PostgreSQL
    last_name: Option<String>,            // Maps to `character varying(255)` in PostgreSQL
    birth_date: Option<chrono::NaiveDate>, // Maps to `date` in PostgreSQL
    is_alive: bool,               // Maps to `boolean` in PostgreSQL
    current_sex: Option<String>,  // Maps to `character varying(50)` in PostgreSQL
    current_alias: Option<String>,// Maps to `character varying(255)` in PostgreSQL
}
#[derive(Serialize)]
struct CreatePersonRow{
    person_id: i32               // Maps to `integer` type in PostgreSQL
}
async fn create_person(
    State(pg_pool): State<PgPool>,
    Json(person): Json<CreatePersonReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(CreatePersonRow, "INSERT INTO people (first_name, last_name, birth_date, is_alive, current_sex, current_alias) VALUES ($1, $2, $3, $4, $5, $6) RETURNING person_id",
    person.first_name,
    person.last_name,
    person.birth_date,
    person.is_alive,
    person.current_sex,
    person.current_alias
    ).fetch_one(&pg_pool)
    .await
    .map_err(|e|{
        (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({ "success": false, "message": e.to_string()}).to_string()
        )
    })?;
    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": row }).to_string()
    ))
}

async fn update_person(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!()
}

async fn delete_person(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!()
}