use sea_orm::{Database, DatabaseConnection};
use std::env;
use dotenv::dotenv;

pub async fn init_db_pool() -> DatabaseConnection {
    dotenv().ok(); // 加载 .env 文件

    // 获取数据库连接 URL
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // 创建数据库连接
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    db
}
