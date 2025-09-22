use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelatedEntity, DeriveRelation,
    EntityTrait, EnumIter, Expr, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, FromRow)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::drawings::Entity")]
    Drawings,
    #[sea_orm(has_many = "super::objects::Entity")]
    Objects,
    #[sea_orm(has_many = "super::project_permissions::Entity")]
    ProjectPermissions,
}

impl Related<super::drawings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Drawings.def()
    }
}

impl Related<super::objects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Objects.def()
    }
}

impl Related<super::project_permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectPermissions.def()
    }
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        super::project_permissions::Relation::Accounts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::project_permissions::Relation::Projects.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::drawings::Entity")]
    Drawings,
    #[sea_orm(entity = "super::objects::Entity")]
    Objects,
    #[sea_orm(entity = "super::project_permissions::Entity")]
    ProjectPermissions,
    #[sea_orm(entity = "super::accounts::Entity")]
    Accounts,
}
