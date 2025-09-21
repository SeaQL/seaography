use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelatedEntity, DeriveRelation,
    EnumIter, Expr, PrimaryKeyTrait,
};
use seaography::CustomFields;
use sqlx::FromRow;
use uuid::Uuid;

use crate::types::{Fill, Shape, Stroke};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, FromRow)]
#[sea_orm(table_name = "objects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub project_id: Uuid,
    pub drawing_id: Uuid,
    #[sea_orm(column_type = "JsonBinary")]
    pub fill: Fill,
    #[sea_orm(column_type = "JsonBinary")]
    pub stroke: Stroke,
    #[sea_orm(column_type = "JsonBinary")]
    pub shape: Shape,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {}

#[CustomFields]
impl Model {
    async fn svg(&self) -> async_graphql::Result<String> {
        Ok("TODO".to_string())
    }
}
