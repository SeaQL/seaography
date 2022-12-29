use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "language")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub language_id: i16,
    pub name: Vec<u8>,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
