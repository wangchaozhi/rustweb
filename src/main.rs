use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod repositories;
mod services;
mod controllers;

use controllers::user_controller::{get_users, get_user_by_id};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // 加载 .env 文件

    // 初始化数据库连接
    let db = db::connect_to_db().await;

    // 启动 HTTP 服务器
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("Starting server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // 将数据库连接注入到应用中
            .service(get_users)
            .service(get_user_by_id)
    })
        .bind(server_address)?
        .run()
        .await
}
