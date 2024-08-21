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
                    .route("/names", get(get_names)) //to do, :person_id for specific person
                    .route("/sex", get(get_sex)) //to do, :person_id for specific person
                    .route("/aliases", get(get_alias)) //to do, :person_id for specific person
                    .route("/guardians", get(get_guardian))//to do, :person_id for specific person
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
    first_parent_id: Option<i32>,
    first_parent_relationship: Option<String>,     // Maps to `character varying(50)`, can be NULL
    second_parent_id: Option<i32>,                  // Maps to `integer`, can be NULL
    second_parent_relationship: Option<String>,    // Maps to `character varying(50)`, can be NULL
    guardian_id: Option<i32>,                       // Maps to `integer`, can be NULL
}

#[derive(Serialize)]
struct NameHistory{
    history_id: i32,
    person_id: i32,
    name: String,
    start_date: NaiveDate
}

#[derive(Serialize)]
struct AliasHistory{
    history_id: i32,
    person_id: i32,
    alias: String,
    start_date: NaiveDate
}

#[derive(Serialize)]
struct SexHistory{
    history_id: i32,
    person_id: i32,
    sex: String,
    start_date: NaiveDate
}

#[derive(Serialize)]
struct GuardianHistory{
    history_id: i32,
    person_id: i32,
    guardian_id: i32,
    start_date: NaiveDate
}

async fn get_people(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(Person, "SELECT person_id, first_name, last_name, birth_date, is_alive, current_sex, current_alias, first_parent_id, first_parent_relationship, second_parent_id, second_parent_relationship, guardian_id  FROM people ORDER BY person_id")
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
async fn get_names(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(NameHistory, "SELECT history_id, person_id, name, start_date FROM name_history ORDER BY history_id")
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

async fn get_sex(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(SexHistory, "SELECT history_id, person_id, sex, start_date FROM sex_history ORDER BY history_id")
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

async fn get_alias(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(AliasHistory, "SELECT history_id, person_id, alias, start_date FROM alias_history ORDER BY history_id")
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
async fn get_guardian(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(GuardianHistory, "SELECT history_id, person_id, guardian_id, start_date FROM guardian_history ORDER BY history_id")
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
    first_parent_id: Option<i32>,
    first_parent_relationship: Option<String>,     // Maps to `character varying(50)`, can be NULL
    second_parent_id: Option<i32>,                  // Maps to `integer`, can be NULL
    second_parent_relationship: Option<String>,    // Maps to `character varying(50)`, can be NULL
    guardian_id: Option<i32>,                       // Maps to `integer`, can be NULL
}
#[derive(Serialize)]
struct CreatePersonRow{
    person_id: i32               // Maps to `integer` type in PostgreSQL
}
async fn create_person(
    State(pg_pool): State<PgPool>,
    Json(person): Json<CreatePersonReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(CreatePersonRow, "INSERT INTO people (first_name, last_name, birth_date, is_alive, current_sex, current_alias, first_parent_id, first_parent_relationship, second_parent_id, second_parent_relationship, guardian_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING person_id",
    person.first_name,
    person.last_name,
    person.birth_date,
    person.is_alive,
    person.current_sex,
    person.current_alias,
    person.first_parent_id,
    person.first_parent_relationship,
    person.second_parent_id,
    person.second_parent_relationship,
    person.guardian_id
    ).fetch_one(&pg_pool)
    .await
    .map_err(|e|{
        (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({ "success": false, "message": e.to_string()}).to_string()
        )
    })?;
    if let Some(first_name) = &person.first_name{
        sqlx::query!(
            "INSERT INTO name_history (person_id, name, start_date)
            VALUES ($1, $2, CURRENT_DATE)", row.person_id, first_name
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        ))?;
    }
    if let Some(current_sex) = &person.current_sex{
        sqlx::query!(
            "INSERT INTO sex_history (person_id, sex, start_date)
            VALUES ($1, $2, CURRENT_DATE)", row.person_id, current_sex
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        ))?;
    }
    if let Some(current_alias) = &person.current_alias{
        sqlx::query!(
            "INSERT INTO alias_history (person_id, alias, start_date)
            VALUES ($1, $2, CURRENT_DATE)", row.person_id, current_alias
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        ))?;
    }
    if let Some(guardian_id) = &person.guardian_id{
        sqlx::query!(
            "INSERT INTO guardian_history (person_id, guardian_id, start_date)
            VALUES ($1, $2, CURRENT_DATE)", row.person_id, guardian_id
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        ))?;
    }
    if let Some(first_parent_id) = &person.first_parent_id {
        // Second parent can be null
        sqlx::query!(
            "INSERT INTO birth_parents (person_id, first_parent_id, second_parent_id, first_parent_relationship, second_parent_relationship)
            VALUES ($1, $2, $3, $4, $5)",
            row.person_id,          
            first_parent_id,
            person.second_parent_id,
            person.first_parent_relationship, 
            person.second_parent_relationship
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        ))?;
    }
    Ok((
        StatusCode::OK,
        json!({ "success": true, "data": row }).to_string()
    ))
}

#[derive(Deserialize)]

struct UpdatePersonReq{
    first_name: Option<String>,           // Maps to `character varying(255)` in PostgreSQL
    last_name: Option<String>,            // Maps to `character varying(255)` in PostgreSQL
    birth_date: Option<chrono::NaiveDate>, // Maps to `date` in PostgreSQL
    is_alive: Option<bool>,               // Maps to `boolean` in PostgreSQL
    current_sex: Option<String>,  // Maps to `character varying(50)` in PostgreSQL
    current_alias: Option<String>,// Maps to `character varying(255)` in PostgreSQL
    guardian_id: Option<i32>
}
async fn update_person(
    State(pg_pool): State<PgPool>,
    Path(person_id): Path<i32>,
    Json(person): Json<UpdatePersonReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut query = "UPDATE people SET person_id = $1".to_owned();
    let mut i = 2;

    if person.first_name.is_some() {
        query.push_str(&format!(", first_name = ${}", i));
        i += 1;
    }

    if person.last_name.is_some() {
        query.push_str(&format!(", last_name = ${}", i));
        i += 1;
    }

    if person.birth_date.is_some() {
        query.push_str(&format!(", birth_date = ${}", i));
        i += 1;
    }

    if person.is_alive.is_some() {
        query.push_str(&format!(", is_alive = ${}", i));
        i += 1;
    }

    if person.current_sex.is_some() {
        query.push_str(&format!(", current_sex = ${}", i));
        i += 1;
    }

    if person.current_alias.is_some() {
        query.push_str(&format!(", current_alias = ${}", i));
        i +=1 ;
    }

    if person.guardian_id.is_some() {
        query.push_str(&format!(", guardian_id = ${}", i));
    }

    query.push_str(&format!(" WHERE person_id = $1"));

    let mut s = sqlx::query(&query).bind(person_id);

    if let Some(first_name) = &person.first_name {
        s = s.bind(first_name);
    }

    if let Some(last_name) = &person.last_name {
        s = s.bind(last_name);
    }

    if let Some(birth_date) = &person.birth_date {
        s = s.bind(birth_date);
    }

    if let Some(is_alive) = person.is_alive {
        s = s.bind(is_alive);
    }

    if let Some(current_sex) = &person.current_sex {
        s = s.bind(current_sex);
    }

    if let Some(current_alias) = &person.current_alias {
        s = s.bind(current_alias);
    }

    if let Some(guardian_id) = person.guardian_id {
        s = s.bind(guardian_id);
    }

    s.execute(&pg_pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;
    if let Some(first_name) = &person.first_name{
        sqlx::query!(
            "INSERT INTO name_history (person_id, name, start_date)
            VALUES ($1, $2, CURRENT_DATE)", person_id, first_name
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        ))?;
    }
    if let Some(current_sex) = &person.current_sex{
        sqlx::query!(
            "INSERT INTO sex_history (person_id, sex, start_date)
            VALUES ($1, $2, CURRENT_DATE)", person_id, current_sex
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        ))?;
    }
    if let Some(current_alias) = &person.current_alias{
        sqlx::query!(
            "INSERT INTO alias_history (person_id, alias, start_date)
            VALUES ($1, $2, CURRENT_DATE)", person_id, current_alias
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        ))?;
    }
    if let Some(guardian_id) = &person.guardian_id{
        sqlx::query!(
            "INSERT INTO guardian_history (person_id, guardian_id, start_date)
            VALUES ($1, $2, CURRENT_DATE)", person_id, guardian_id
        )
        .execute(&pg_pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        ))?;
    }
    

    Ok((StatusCode::OK, json!({"success": true}).to_string()))
}


async fn delete_person(
    State(pg_pool): State<PgPool>,
    Path(person_id): Path<i32>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!("DELETE FROM people WHERE person_id = $1", person_id)
        .execute(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode:: INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string()}).to_string()
            )
        })?;

    Ok((StatusCode::OK, json!({ "success": true}).to_string()))
}