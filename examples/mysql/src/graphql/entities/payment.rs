use sea_orm::prelude::*;
pub fn filter_recursive(root_filter: Option<Filter>) -> sea_orm::Condition {
    let mut condition = sea_orm::Condition::all();
    if let Some(current_filter) = root_filter {
        if let Some(or_filters) = current_filter.or {
            let or_condition = or_filters
                .into_iter()
                .fold(sea_orm::Condition::any(), |fold_condition, filter| {
                    fold_condition.add(filter_recursive(Some(*filter)))
                });
            condition = condition.add(or_condition);
        }
        if let Some(and_filters) = current_filter.and {
            let and_condition = and_filters
                .into_iter()
                .fold(sea_orm::Condition::all(), |fold_condition, filter| {
                    fold_condition.add(filter_recursive(Some(*filter)))
                });
            condition = condition.add(and_condition);
        }
        if let Some(payment_id) = current_filter.payment_id {
            if let Some(eq_value) = payment_id.eq {
                condition = condition.add(Column::PaymentId.eq(eq_value))
            }
            if let Some(ne_value) = payment_id.ne {
                condition = condition.add(Column::PaymentId.ne(ne_value))
            }
        }
        if let Some(customer_id) = current_filter.customer_id {
            if let Some(eq_value) = customer_id.eq {
                condition = condition.add(Column::CustomerId.eq(eq_value))
            }
            if let Some(ne_value) = customer_id.ne {
                condition = condition.add(Column::CustomerId.ne(ne_value))
            }
        }
        if let Some(staff_id) = current_filter.staff_id {
            if let Some(eq_value) = staff_id.eq {
                condition = condition.add(Column::StaffId.eq(eq_value))
            }
            if let Some(ne_value) = staff_id.ne {
                condition = condition.add(Column::StaffId.ne(ne_value))
            }
        }
        if let Some(rental_id) = current_filter.rental_id {
            if let Some(eq_value) = rental_id.eq {
                condition = condition.add(Column::RentalId.eq(eq_value))
            }
            if let Some(ne_value) = rental_id.ne {
                condition = condition.add(Column::RentalId.ne(ne_value))
            }
        }
        if let Some(amount) = current_filter.amount {
            if let Some(eq_value) = amount.eq {
                condition = condition.add(Column::Amount.eq(eq_value))
            }
            if let Some(ne_value) = amount.ne {
                condition = condition.add(Column::Amount.ne(ne_value))
            }
        }
        if let Some(payment_date) = current_filter.payment_date {
            if let Some(eq_value) = payment_date.eq {
                condition = condition.add(Column::PaymentDate.eq(eq_value))
            }
            if let Some(ne_value) = payment_date.ne {
                condition = condition.add(Column::PaymentDate.ne(ne_value))
            }
        }
        if let Some(last_update) = current_filter.last_update {
            if let Some(eq_value) = last_update.eq {
                condition = condition.add(Column::LastUpdate.eq(eq_value))
            }
            if let Some(ne_value) = last_update.ne {
                condition = condition.add(Column::LastUpdate.ne(ne_value))
            }
        }
    }
    condition
}
use crate::graphql::*;
pub use crate::orm::payment::*;
#[async_graphql::Object(name = "Payment")]
impl Model {
    pub async fn payment_id(&self) -> &u16 {
        &self.payment_id
    }
    pub async fn customer_id(&self) -> &u16 {
        &self.customer_id
    }
    pub async fn staff_id(&self) -> &u8 {
        &self.staff_id
    }
    pub async fn rental_id(&self) -> &Option<i32> {
        &self.rental_id
    }
    pub async fn amount(&self) -> &Decimal {
        &self.amount
    }
    pub async fn payment_date(&self) -> &DateTime {
        &self.payment_date
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
        &self.last_update
    }
    pub async fn customer_customer<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::customer::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CustomerCustomerFK(self.customer_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn rental_rental<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<crate::orm::rental::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = RentalRentalFK(self.rental_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn staff_staff<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::staff::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = StaffStaffFK(self.staff_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "PaymentFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub payment_id: Option<TypeFilter<u16>>,
    pub customer_id: Option<TypeFilter<u16>>,
    pub staff_id: Option<TypeFilter<u8>>,
    pub rental_id: Option<TypeFilter<i32>>,
    pub amount: Option<TypeFilter<Decimal>>,
    pub payment_date: Option<TypeFilter<DateTime>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CustomerCustomerFK(u16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CustomerCustomerFK> for OrmDataloader {
    type Value = crate::orm::customer::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CustomerCustomerFK],
    ) -> Result<std::collections::HashMap<CustomerCustomerFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::customer::Column::CustomerId.as_column_ref(),
                )
                .into_simple_expr(),
            ])),
            sea_orm::sea_query::BinOper::In,
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(
                keys.iter()
                    .map(|tuple| {
                        sea_orm::sea_query::SimpleExpr::Values(vec![tuple.0.clone().into()])
                    })
                    .collect(),
            )),
        ));
        Ok(crate::orm::customer::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CustomerCustomerFK(model.customer_id.clone());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RentalRentalFK(Option<i32>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<RentalRentalFK> for OrmDataloader {
    type Value = crate::orm::rental::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[RentalRentalFK],
    ) -> Result<std::collections::HashMap<RentalRentalFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::rental::Column::RentalId.as_column_ref())
                    .into_simple_expr(),
            ])),
            sea_orm::sea_query::BinOper::In,
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(
                keys.iter()
                    .map(|tuple| {
                        sea_orm::sea_query::SimpleExpr::Values(vec![tuple.0.clone().into()])
                    })
                    .collect(),
            )),
        ));
        Ok(crate::orm::rental::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = RentalRentalFK(Some(model.rental_id).clone());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StaffStaffFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StaffStaffFK> for OrmDataloader {
    type Value = crate::orm::staff::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StaffStaffFK],
    ) -> Result<std::collections::HashMap<StaffStaffFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::staff::Column::StaffId.as_column_ref())
                    .into_simple_expr(),
            ])),
            sea_orm::sea_query::BinOper::In,
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(
                keys.iter()
                    .map(|tuple| {
                        sea_orm::sea_query::SimpleExpr::Values(vec![tuple.0.clone().into()])
                    })
                    .collect(),
            )),
        ));
        Ok(crate::orm::staff::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = StaffStaffFK(model.staff_id.clone());
                (key, model)
            })
            .collect())
    }
}
