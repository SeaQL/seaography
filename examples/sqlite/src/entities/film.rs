use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "film")]
#[graphql(complex)]
#[graphql(name = "Film")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub film_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub release_year: Option<String>,
    pub language_id: i16,
    pub original_language_id: Option<i16>,
    pub rental_duration: i16,
    #[sea_orm(column_type = "Decimal(Some((4, 2)))")]
    pub rental_rate: Decimal,
    pub length: Option<i16>,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub replacement_cost: Decimal,
    pub rating: Option<String>,
    pub special_features: Option<String>,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::language::Entity",
        from = "Column::OriginalLanguageId",
        to = "super::language::Column::LanguageId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Language2,
    #[sea_orm(
        belongs_to = "super::language::Entity",
        from = "Column::LanguageId",
        to = "super::language::Column::LanguageId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Language1,
    #[sea_orm(has_many = "super::film_actor::Entity")]
    FilmActor,
    #[sea_orm(has_many = "super::film_category::Entity")]
    FilmCategory,
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
}

impl Related<super::film_actor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmActor.def()
    }
}

impl Related<super::film_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmCategory.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
