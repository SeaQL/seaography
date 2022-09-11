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
    #[sea_orm(primary_key, column_type = "Custom(\"MEDIUMINT UNSIGNED\".to_owned())")]
    pub inventory_id: String,
    pub film_id: u16,
    pub store_id: u8,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::film::Entity",
        from = "Column::FilmId",
        to = "super::film::Column::FilmId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Film,
    #[sea_orm(
        belongs_to = "super::store::Entity",
        from = "Column::StoreId",
        to = "super::store::Column::StoreId",
        on_update = "Cascade",
        on_delete = "Restrict"
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
