use sea_orm::{Database, DatabaseConnection};
use std::env;

/// 初始化数据库连接
pub async fn connect_to_db() -> DatabaseConnection {
    // 从环境变量加载数据库 URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    Database::connect(&database_url).await.expect("Failed to connect to the database")
}
