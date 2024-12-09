use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "staff")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub staff_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub address_id: i16,
    pub email: Option<String>,
    pub store_id: i16,
    pub active: bool,
    pub username: String,
    pub password: Option<String>,
    pub last_update: DateTime,
    #[sea_orm(column_type = "Binary(255)", nullable)]
    pub picture: Option<Vec<u8>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::address::Entity",
        from = "Column::AddressId",
        to = "super::address::Column::AddressId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Address,
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
    #[sea_orm(has_many = "super::rental::Entity")]
    Rental,
    #[sea_orm(
        belongs_to = "super::store::Entity",
        from = "Column::StoreId",
        to = "super::store::Column::StoreId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Store,
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl Related<super::rental::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rental.def()
    }
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::address::Entity")]
    Address,
    #[sea_orm(entity = "super::payment::Entity")]
    Payment,
    #[sea_orm(entity = "super::rental::Entity")]
    Rental,
    #[sea_orm(entity = "super::store::Entity")]
    Store,
}
