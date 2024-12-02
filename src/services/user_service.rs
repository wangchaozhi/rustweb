use crate::repositories::user_repository::UserRepository;
use crate::models::users::Model as UserModel;
use sea_orm::DatabaseConnection;

pub struct UserService;

impl UserService {
    /// 获取所有用户
    pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<UserModel>, String> {
        UserRepository::get_all_users(db)
            .await
            .map_err(|err| format!("Failed to fetch users: {}", err))
    }

    /// 根据 ID 获取用户
    pub async fn get_user_by_id(db: &DatabaseConnection, user_id: i32) -> Result<Option<UserModel>, String> {
        UserRepository::get_user_by_id(db, user_id)
            .await
            .map_err(|err| format!("Failed to fetch user: {}", err))
    }


    pub async fn create_user(
        db: &DatabaseConnection,
        user: UserModel,
    ) -> Result<i32, String> {
        UserRepository::create_user(db, user)
            .await
            .map_err(|err| format!("Failed to create user: {}", err))
    }
}