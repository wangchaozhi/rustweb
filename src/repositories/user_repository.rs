use crate::models::users::{Entity as User, ActiveModel, Model as UserModel};
use sea_orm::{DatabaseConnection, EntityTrait, Set, InsertResult};

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

    /// 创建新用户并返回自增 ID
    pub async fn create_user(
        db: &DatabaseConnection,
        user: UserModel,
    ) -> Result<i32, sea_orm::DbErr> {
        // 将 UserModel 转换为 ActiveModel 以进行插入操作
        let user_active_model = ActiveModel {
            name: Set(user.name), // 使用 Set 来插入值
            email: Set(user.email), // 同样是 Set，用于插入 email 字段
            ..Default::default() // 其他字段使用默认值
        };

        // 执行插入操作，并返回 InsertResult
        let insert_result: InsertResult<ActiveModel> = User::insert(user_active_model)
            .exec(db) // 执行插入操作
            .await?;

        // 返回插入后的自增 ID
        Ok(insert_result.last_insert_id) // 返回自增 ID
    }
}
