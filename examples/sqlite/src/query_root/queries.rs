use super::*;
use async_graphql::Result as GqlResult;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use seaography::{apply_pagination, macros::CustomOperation, Connection, PaginationInput};

/*

{
  customer_of_store2(pagination: { page: { page: 0, limit: 1 } }) {
    nodes {
      storeId
      customerId
      lastName
      email
    }
    paginationInfo {
      pages
      current
    }
  }
}

*/

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(CustomOperation)]
pub struct Operations {
    customer_of_store2: fn(pagination: PaginationInput) -> Connection::<customer::Entity>,
}

impl Operations {
    async fn customer_of_store2(
        ctx: &ResolverContext<'_>,
        pagination: PaginationInput,
    ) -> GqlResult<Connection<customer::Entity>> {
        let db = ctx.data::<DatabaseConnection>()?;

        let query = customer::Entity::find().filter(customer::Column::StoreId.eq(2));
        let connection = apply_pagination::<customer::Entity, _>(db, query, pagination).await?;

        Ok(connection)
    }
}

/*
fn customer_of_store2() -> Field {
    use seaography::GqlModelType;

    Field::new(
        "customer_of_store2",
        seaography::Connection::<customer::Entity>::gql_type_ref(&CONTEXT),
        move |ctx| {
            FieldFuture::new(async move {
                use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
                use seaography::{apply_pagination, PaginationInputBuilder};

                let db = ctx.data::<DatabaseConnection>()?;

                let stmt = customer::Entity::find().filter(customer::Column::StoreId.eq(2));
                let pagination = ctx.args.get("pagination");
                let pagination =
                    PaginationInputBuilder { context: &CONTEXT }.parse_object(pagination);
                let connection = apply_pagination::<customer::Entity>(db, stmt, pagination).await?;

                Ok(Some(FieldValue::owned_any(connection)))
            })
        },
    )
    .argument(InputValue::new(
        "pagination",
        seaography::PaginationInput::gql_type_ref(&CONTEXT),
    ))
}
*/
