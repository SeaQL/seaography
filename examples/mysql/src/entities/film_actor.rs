use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "film_actor")]
#[graphql(complex)]
#[graphql(name = "FilmActor")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub actor_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub film_id: i32,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::actor::Entity",
        from = "Column::ActorId",
        to = "super::actor::Column::ActorId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Actor,
    #[sea_orm(
        belongs_to = "super::film::Entity",
        from = "Column::FilmId",
        to = "super::film::Column::FilmId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Film,
}

impl Related<super::actor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Actor.def()
    }
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Film.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
