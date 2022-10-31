use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "city")]
// #[graphql(complex)]
#[graphql(name = "City")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub city_id: i32,
    pub city: String,
    pub country_id: i16,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation
// , seaography::macros::RelationsCompact
)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::country::Entity",
        from = "Column::CountryId",
        to = "super::country::Column::CountryId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Country,
    #[sea_orm(has_many = "super::address::Entity")]
    Address,
}

impl Related<super::country::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Country.def()
    }
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
