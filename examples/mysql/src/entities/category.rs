use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "category")]
#[graphql(complex)]
#[graphql(name = "Category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub category_id: u8,
    pub name: String,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::film_category::Entity")]
    FilmCategory,
}

impl Related<super::film_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmCategory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
