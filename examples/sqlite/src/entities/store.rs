use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "store")]
// #[graphql(complex)]
#[graphql(name = "Store")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub store_id: i32,
    pub manager_staff_id: i16,
    pub address_id: i32,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation,
    // seaography::macros::RelationsCompact
)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::address::Entity",
        from = "Column::AddressId",
        to = "super::address::Column::AddressId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Address,
    #[sea_orm(
        belongs_to = "super::staff::Entity",
        from = "Column::ManagerStaffId",
        to = "super::staff::Column::StaffId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Staff,
    #[sea_orm(has_many = "super::customer::Entity")]
    Customer,
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl Related<super::staff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staff.def()
    }
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

impl ActiveModelBehavior for ActiveModel {}
