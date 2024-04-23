use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use axum::{response::IntoResponse, routing::{delete, get, post, put}, Json, Router};

use crate::models::Student;
mod models;
mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Database Url not set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Falied to establish the connection!"))
}


#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/add",put(add_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn add_user(Json(body): Json<Student>) -> impl IntoResponse {
    use self::schema::student::dsl::*;

    let mut conn = establish_connection();

    // Extract data from the JSON body
    let new_student = body;

    // Insert the new student into the database
    diesel::insert_into(student)
        .values(&new_student)
        .execute(&mut conn)
        .expect("Failed to insert student into database");

    
    "User added successfully"
}


