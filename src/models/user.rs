use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: Option<String>, // 使用 Option 表示可以为 NULL
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl sea_orm::RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        panic!("No relations defined for this entity")
    }
}

impl ActiveModelBehavior for ActiveModel {}
