use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "inventory")]
#[graphql(complex)]
#[graphql(name = "Inventory")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub inventory_id: i32,
    pub film_id: i32,
    pub store_id: i32,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::film::Entity",
        from = "Column::FilmId",
        to = "super::film::Column::FilmId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Film,
    #[sea_orm(
        belongs_to = "super::store::Entity",
        from = "Column::StoreId",
        to = "super::store::Column::StoreId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Store,
    #[sea_orm(has_many = "super::rental::Entity")]
    Rental,
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Film.def()
    }
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl Related<super::rental::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rental.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
