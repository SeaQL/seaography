use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "actor")]
#[graphql(complex)]
#[graphql(name = "Actor")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub actor_id: u16,
    pub first_name: String,
    pub last_name: String,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::film_actor::Entity")]
    FilmActor,
}

impl Related<super::film_actor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmActor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
