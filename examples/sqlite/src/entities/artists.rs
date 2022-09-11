use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "artists")]
#[graphql(complex)]
#[graphql(name = "Artists")]
pub struct Model {
    #[sea_orm(column_name = "ArtistId", primary_key)]
    pub artist_id: i32,
    #[sea_orm(column_name = "Name")]
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::albums::Entity")]
    Albums,
}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Albums.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
