use super::*;
use async_graphql::Result as GqlResult;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use seaography::{apply_pagination, macros::CustomOperation, Connection, PaginationInput};

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(CustomOperation)]
pub struct Operations {
    customer_of_store2: fn(pagination: PaginationInput) -> Connection::<customer::Entity>,
    staff_by_id: fn(id: i16) -> Option<staff::Model>,
    many_rental: fn() -> Vec<rental::Model>,
    purchase_order: fn() -> custom_output::PurchaseOrder,
}

impl Operations {
    async fn customer_of_store2(
        ctx: &ResolverContext<'_>,
        pagination: PaginationInput,
    ) -> GqlResult<Connection<customer::Entity>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let query = customer::Entity::find().filter(customer::Column::StoreId.eq(2));
        let connection = apply_pagination(&CONTEXT, db, query, pagination).await?;

        Ok(connection)
    }

    async fn staff_by_id(ctx: &ResolverContext<'_>, id: i16) -> GqlResult<Option<staff::Model>> {
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(staff::Entity::find_by_id(id).one(db).await?)
    }

    async fn many_rental(ctx: &ResolverContext<'_>) -> GqlResult<Vec<rental::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Ok(rental::Entity::find().limit(10).all(db).await?)
    }

    async fn purchase_order(_ctx: &ResolverContext<'_>) -> GqlResult<custom_output::PurchaseOrder> {
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
