use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "invoices")]
#[graphql(complex)]
#[graphql(name = "Invoices")]
pub struct Model {
    #[sea_orm(column_name = "InvoiceId", primary_key)]
    pub invoice_id: i32,
    #[sea_orm(column_name = "CustomerId")]
    pub customer_id: i32,
    #[sea_orm(column_name = "InvoiceDate")]
    pub invoice_date: DateTime,
    #[sea_orm(column_name = "BillingAddress")]
    pub billing_address: Option<String>,
    #[sea_orm(column_name = "BillingCity")]
    pub billing_city: Option<String>,
    #[sea_orm(column_name = "BillingState")]
    pub billing_state: Option<String>,
    #[sea_orm(column_name = "BillingCountry")]
    pub billing_country: Option<String>,
    #[sea_orm(column_name = "BillingPostalCode")]
    pub billing_postal_code: Option<String>,
    #[sea_orm(column_name = "Total")]
    pub total: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::customers::Entity",
        from = "Column::CustomerId",
        to = "super::customers::Column::CustomerId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Customers,
    #[sea_orm(has_many = "super::invoice_items::Entity")]
    InvoiceItems,
}

impl Related<super::customers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customers.def()
    }
}

impl Related<super::invoice_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InvoiceItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
