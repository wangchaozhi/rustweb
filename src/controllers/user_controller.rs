use crate::services::user_service::UserService;
use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

/// 处理获取所有用户的请求
#[get("/users")]
async fn get_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    match UserService::get_all_users(&db).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

/// 处理根据 ID 获取用户的请求
#[get("/users/{id}")]
async fn get_user_by_id(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>,
) -> impl Responder {
    match UserService::get_user_by_id(&db, user_id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
