use crate::models::user::{Entity as User, Model as UserModel};
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct UserRepository;

impl UserRepository {
    /// 获取所有用户
    pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<UserModel>, sea_orm::DbErr> {
        User::find().all(db).await
    }

    /// 根据 ID 获取用户
    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Option<UserModel>, sea_orm::DbErr> {
        User::find_by_id(user_id).one(db).await
    }
}
