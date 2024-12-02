use crate::services::user_service::UserService;
use crate::models::users::Model as UserModel;
use actix_web::{get, post,web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

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


#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: Option<String>,
}

/// 处理创建用户请求
#[post("/users")]
async fn create_user(
    db: web::Data<DatabaseConnection>,
    new_user: web::Json<CreateUserRequest>,
) -> impl Responder {
    let user = UserModel {
        id: 0, // id will be auto-generated
        name: new_user.name.clone(),
        email: new_user.email.clone(),
    };
    match UserService::create_user(&db, user).await {
        Ok(user_id) => HttpResponse::Created().json(user_id),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}