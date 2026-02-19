use axum::extract::Query;
use axum::{extract::Path};
use axum::{Json, Router};
use serde::{Deserialize};
use tokio::net::TcpListener;
use axum::routing::{get, post};
use git::open_repo;
use template::load_template;
use config::AppConfig;


async fn get_users() -> String {
    String::from("Hello, World!")
}

async fn get_profile() -> String {
    String::from("Profile, Fachru!")
}

async fn detail_user(Path(id) : Path<i32>) -> String {
    format!("user id: {}", id)
}

#[derive(Deserialize)]
struct Pagination {
    page: u32,
    size: u32,
}

#[derive(Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

async fn index_user(Query(pagination): Query<Pagination>) -> String {
    format!("menampilkan data user dari page: {}, size: {}", pagination.page, pagination.size)
}

async fn create_user(Json(payload): Json<User>) -> String {
    format!("user id: {}, name: {}, email: {}", payload.id, payload.name, payload.email)
}

#[tokio::main]
async fn main() {
    let app_router = Router::new()
        .route("/", get(get_users))
        .route("/profile", get(get_profile))
        .route("/{id}", get(detail_user))
        .route("/index", get(index_user))
        .route("/create", post(create_user));

    let app = Router::new()
        .nest("/v1/users", app_router);
    //localhost:4000
    let listener = TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}