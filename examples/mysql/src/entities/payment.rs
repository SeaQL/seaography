use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "payment")]
#[graphql(complex)]
#[graphql(name = "Payment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub payment_id: i32,
    pub customer_id: i32,
    pub staff_id: i32,
    pub rental_id: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub amount: Decimal,
    pub payment_date: DateTime,
    pub last_update: Option<DateTimeUtc>,
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
        belongs_to = "super::rental::Entity",
        from = "Column::RentalId",
        to = "super::rental::Column::RentalId",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Rental,
    #[sea_orm(
        belongs_to = "super::staff::Entity",
        from = "Column::StaffId",
        to = "super::staff::Column::StaffId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Staff,
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::rental::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rental.def()
    }
}

impl Related<super::staff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staff.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
