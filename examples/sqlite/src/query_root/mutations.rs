use super::*;
use async_graphql::{Result as GqlResult, Upload};
use custom_inputs::RentalRequest;
use sea_orm::{DbErr, EntityTrait};
use seaography::macros::CustomOperation;

#[allow(dead_code)]
#[derive(CustomOperation)]
pub struct Operations {
    foo: fn(username: String) -> String,
    bar: fn(x: i32, y: i32) -> i32,
    login: fn() -> customer::Model,
    rental_request: fn(rental_request: RentalRequest) -> String,
    upload: fn(upload: Upload) -> String,
    maybe_rental_request: fn(rental_request: Option<RentalRequest>) -> Option<rental::Model>,
    many_rental_request: fn(rental_requests: Vec<RentalRequest>) -> i32,
}

impl Operations {
    async fn upload(ctx: &ResolverContext<'_>, upload: Upload) -> GqlResult<String> {
        Ok(format!(
            "upload: filename={}",
            upload.value(ctx).unwrap().filename
        ))
    }

    async fn foo(_ctx: &ResolverContext<'_>, username: String) -> GqlResult<String> {
        Ok(format!("Hello, {}!", username))
    }

    async fn bar(_ctx: &ResolverContext<'_>, x: i32, y: i32) -> GqlResult<i32> {
        Ok(x + y)
    }

    async fn login(ctx: &ResolverContext<'_>) -> GqlResult<customer::Model> {
        use sea_orm::EntityTrait;

        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(customer::Entity::find()
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("Customer not found".to_owned()))?)
    }

    async fn rental_request(
        _ctx: &ResolverContext<'_>,
        rental_request: RentalRequest,
    ) -> GqlResult<String> {
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
        ctx: &ResolverContext<'_>,
        rental_request: Option<RentalRequest>,
    ) -> GqlResult<Option<rental::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Ok(match rental_request {
            Some(_) => rental::Entity::find().one(db).await?,
            None => None,
        })
    }

    async fn many_rental_request(
        _ctx: &ResolverContext<'_>,
        rental_requests: Vec<RentalRequest>,
    ) -> GqlResult<i32> {
        Ok(rental_requests.len() as i32)
    }
}
