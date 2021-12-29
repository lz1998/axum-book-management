use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::database::orm::get_conn;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "book")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub name: String,
    pub operator: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn create(name: &str, operator: &str) -> anyhow::Result<()> {
    let conn = get_conn().await;
    let model = ActiveModel {
        name: Set(name.into()),
        operator: Set(operator.into()),
        ..Default::default()
    };
    model.insert(conn).await.map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

pub async fn search(query: &str) -> anyhow::Result<Vec<Model>> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::Name.contains(query))
        .all(conn)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn update(id: i32, name: &str, operator: &str) -> anyhow::Result<()> {
    let conn = get_conn().await;
    Entity::update_many()
        .col_expr(Column::Name, Expr::value(name))
        .col_expr(Column::Operator, Expr::value(operator))
        .filter(Column::Id.eq(id))
        .exec(conn)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

pub async fn delete(id: i32) -> anyhow::Result<()> {
    let conn = get_conn().await;
    Entity::delete_many()
        .filter(Column::Id.eq(id))
        .exec(conn)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_book() {
        dotenv::dotenv().ok();
        create("test", "lz1998").await.expect("create book failed");
    }

    #[tokio::test]
    async fn test_search_book() {
        dotenv::dotenv().ok();
        let books = search("2").await.expect("search book failed");
        for book in books {
            println!("{:?}", book);
        }
    }

    #[tokio::test]
    async fn test_update_book() {
        dotenv::dotenv().ok();
        update(1, "test2", "lz1998")
            .await
            .expect("update book failed");
    }

    #[tokio::test]
    async fn test_delete_book() {
        dotenv::dotenv().ok();
        delete(1).await.expect("delete book failed");
    }
}
