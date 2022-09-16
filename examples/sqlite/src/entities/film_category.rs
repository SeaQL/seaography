use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "film_category")]
#[graphql(complex)]
#[graphql(name = "FilmCategory")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub film_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i16,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::CategoryId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::film::Entity",
        from = "Column::FilmId",
        to = "super::film::Column::FilmId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Film,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Film.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
