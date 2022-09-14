use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "rental")]
#[graphql(complex)]
#[graphql(name = "Rental")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub rental_id: i32,
    pub rental_date: DateTime,
    pub inventory_id: i32,
    pub customer_id: i16,
    pub return_date: Option<DateTime>,
    pub staff_id: i16,
    pub last_update: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::customer::Entity",
        from = "Column::CustomerId",
        to = "super::customer::Column::CustomerId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Customer,
    #[sea_orm(
        belongs_to = "super::inventory::Entity",
        from = "Column::InventoryId",
        to = "super::inventory::Column::InventoryId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Inventory,
    #[sea_orm(
        belongs_to = "super::staff::Entity",
        from = "Column::StaffId",
        to = "super::staff::Column::StaffId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Staff,
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl Related<super::staff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staff.def()
    }
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
