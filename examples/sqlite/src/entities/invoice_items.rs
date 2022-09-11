use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "invoice_items")]
#[graphql(complex)]
#[graphql(name = "InvoiceItems")]
pub struct Model {
    #[sea_orm(column_name = "InvoiceLineId", primary_key)]
    pub invoice_line_id: i32,
    #[sea_orm(column_name = "InvoiceId")]
    pub invoice_id: i32,
    #[sea_orm(column_name = "TrackId")]
    pub track_id: i32,
    #[sea_orm(column_name = "UnitPrice")]
    pub unit_price: f64,
    #[sea_orm(column_name = "Quantity")]
    pub quantity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tracks::Entity",
        from = "Column::TrackId",
        to = "super::tracks::Column::TrackId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Tracks,
    #[sea_orm(
        belongs_to = "super::invoices::Entity",
        from = "Column::InvoiceId",
        to = "super::invoices::Column::InvoiceId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Invoices,
}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tracks.def()
    }
}

impl Related<super::invoices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoices.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
