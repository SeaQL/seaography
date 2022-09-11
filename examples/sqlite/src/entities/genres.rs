use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "genres")]
#[graphql(complex)]
#[graphql(name = "Genres")]
pub struct Model {
    #[sea_orm(column_name = "GenreId", primary_key)]
    pub genre_id: i32,
    #[sea_orm(column_name = "Name")]
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::tracks::Entity")]
    Tracks,
}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tracks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
