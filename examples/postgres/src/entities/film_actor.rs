use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "film_actor")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub actor_id: i16,
    #[sea_orm(primary_key, auto_increment = false)]
    pub film_id: i16,
    pub last_update: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(
        entity = "super::actor::Entity",
        active_model = "super::actor::ActiveModel"
    )]
    Actor,
    #[sea_orm(
        entity = "super::film::Entity",
        active_model = "super::film::ActiveModel"
    )]
    Film,
}