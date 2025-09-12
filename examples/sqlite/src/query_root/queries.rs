use super::*;
use async_graphql;
use async_graphql::Context;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use seaography::{apply_pagination, Connection, CustomFields, PaginationInput};

pub struct Operations;

#[CustomFields]
impl Operations {
    async fn customer_of_store2(
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> async_graphql::Result<Connection<customer::Entity>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let query = customer::Entity::find().filter(customer::Column::StoreId.eq(2));
        let connection = apply_pagination(&CONTEXT, db, query, pagination).await?;

        Ok(connection)
    }

    async fn staff_by_id(
        ctx: &Context<'_>,
        id: i16,
    ) -> async_graphql::Result<Option<staff::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(staff::Entity::find_by_id(id).one(db).await?)
    }

    async fn many_rental(ctx: &Context<'_>) -> async_graphql::Result<Vec<rental::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Ok(rental::Entity::find().limit(10).all(db).await?)
    }

    async fn purchase_order(
        _ctx: &Context<'_>,
    ) -> async_graphql::Result<custom_output::PurchaseOrder> {
        Ok(custom_output::PurchaseOrder {
            po_number: "AB1234".into(),
            lineitems: vec![
                custom_output::Lineitem {
                    product: "Towel".into(),
                    quantity: 2.0,
                    size: Some(custom_output::ProductSize { size: 4 }),
                },
                custom_output::Lineitem {
                    product: "Soap".into(),
                    quantity: 2.5,
                    size: None,
                },
            ],
        })
    }
}
