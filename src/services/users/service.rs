use actix_web::{get, post, put, delete, web, HttpResponse, Reponder};
use super::models::{AllUsers, RegisterUser, UpdateUser};
use crate::AppState;
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::{Pool, Postgres};

#[get("/users")]
async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM Users")
        .fetch_all(&app_state.postgres_client)
        .await;
}