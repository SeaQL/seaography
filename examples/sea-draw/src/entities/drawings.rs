use async_graphql::Context;
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use seaography::CustomFields;

use crate::{InProject, backend::Backend};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "drawings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime<Utc>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub project_id: Uuid,

    #[sea_orm(column_type = "Text")]
    pub name: String,

    pub width: i64,
    pub height: i64,
    // #[sea_orm(column_type = "JsonBinary")]
    // pub data: serde_json::Value,
    // pub tags: Vec<String>,
}

impl InProject for Model {
    fn project_id(&self) -> Uuid {
        self.project_id
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::objects::Entity")]
    Objects,
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::ProjectId",
        to = "super::projects::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Projects,
}

impl Related<super::objects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Objects.def()
    }
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::objects::Entity")]
    Objects,
    #[sea_orm(entity = "super::projects::Entity")]
    Projects,
}

#[CustomFields]
impl Model {
    async fn svg(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let backend = ctx.data::<Backend>()?;
        let objects = crate::entities::objects::Entity::find()
            .filter(crate::entities::objects::Column::DrawingId.eq(self.id))
            .filter(crate::entities::objects::Column::DeletedAt.is_null())
            .all(&backend.db)
            .await?;

        let mut svg = String::new();

        svg.push_str("<?xml version=\"1.0\" standalone=\"no\"?>\n");
        svg.push_str(&format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">\n",
            self.width, self.height));

        for object in objects.iter() {
            svg.push_str("  ");
            svg.push_str(&object.shape.to_svg(&object.fill, &object.stroke));
            svg.push('\n');
        }
        svg.push_str("</svg>\n");
        Ok(svg)
    }
}
