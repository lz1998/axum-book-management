use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::database::orm::get_conn;
use crate::error::CustomResult;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub username: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn create(username: &str, password: &str) -> CustomResult<()> {
    let conn = get_conn().await;
    let model = ActiveModel {
        username: Set(username.into()),
        password: Set(password.into()),
    };
    model.insert(conn).await?;
    Ok(())
}

pub async fn find(username: &str) -> CustomResult<Option<Model>> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::Username.eq(username))
        .one(conn)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        dotenv::dotenv().ok();
        create("test", "123456").await.expect("create user failed");
    }

    #[tokio::test]
    async fn test_find_user() {
        dotenv::dotenv().ok();
        let user = find("test")
            .await
            .expect("find user failed")
            .expect("user not found");
        assert_eq!(user.password, "123456");
    }
}
