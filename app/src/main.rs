mod controller;
mod repo;
mod service;

use actix_web::{App, HttpServer, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server");

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn index() -> String {
    String::from("Hello, world!")
}

async fn create_user(user: web::Json<User>) -> String {
    format!("User created: {}", user.name)
}

async fn update_user(user: web::Json<User>) -> String {
    format!("User updated: {}", user.name)
}
