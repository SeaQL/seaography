use super::*;
use async_graphql;
use async_graphql::Context;
use async_graphql::Upload;
use custom_inputs::RentalRequest;
use sea_orm::{DbErr, EntityTrait};
use seaography::CustomFields;

pub struct Operations;

#[CustomFields]
impl Operations {
    async fn upload(ctx: &Context<'_>, upload: Upload) -> async_graphql::Result<String> {
        Ok(format!(
            "upload: filename={}",
            upload.value(ctx).unwrap().filename
        ))
    }

    async fn foo(_ctx: &Context<'_>, username: String) -> async_graphql::Result<String> {
        Ok(format!("Hello, {}!", username))
    }

    async fn bar(_ctx: &Context<'_>, x: i32, y: i32) -> async_graphql::Result<i32> {
        Ok(x + y)
    }

    async fn login(ctx: &Context<'_>) -> async_graphql::Result<customer::Model> {
        use sea_orm::EntityTrait;

        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(customer::Entity::find()
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("Customer not found".to_owned()))?)
    }

    async fn rental_request(
        _ctx: &Context<'_>,
        rental_request: RentalRequest,
    ) -> async_graphql::Result<String> {
        let mut s = format!(
            "{} wants to rent {}",
            rental_request.customer, rental_request.film
        );
        if let Some(location) = rental_request.location {
            use std::fmt::Write;
            write!(&mut s, " (at {}", location.city).unwrap();
            if let Some(county) = location.county {
                write!(&mut s, ", {}", county).unwrap();
            }
            write!(&mut s, ")").unwrap();
        }
        Ok(s)
    }

    async fn maybe_rental_request(
        ctx: &Context<'_>,
        rental_request: Option<RentalRequest>,
    ) -> async_graphql::Result<Option<rental::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Ok(match rental_request {
            Some(_) => rental::Entity::find().one(db).await?,
            None => None,
        })
    }

    async fn many_rental_request(
        _ctx: &Context<'_>,
        rental_requests: Vec<RentalRequest>,
    ) -> async_graphql::Result<i32> {
        Ok(rental_requests.len() as i32)
    }
}
