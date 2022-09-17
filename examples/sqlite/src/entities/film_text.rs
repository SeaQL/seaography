use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "film_text")]
#[graphql(complex)]
#[graphql(name = "FilmText")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub film_id: i16,
    pub title: String,
    pub description: Option<Vec<u8>>,
}

#[derive(Copy, Clone, Debug, EnumIter, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
