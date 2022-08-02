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
        if let Some(rental_id) = current_filter.rental_id {
            if let Some(eq_value) = rental_id.eq {
                condition = condition.add(Column::RentalId.eq(eq_value))
            }
            if let Some(ne_value) = rental_id.ne {
                condition = condition.add(Column::RentalId.ne(ne_value))
            }
        }
        if let Some(rental_date) = current_filter.rental_date {
            if let Some(eq_value) = rental_date.eq {
                condition = condition.add(Column::RentalDate.eq(eq_value))
            }
            if let Some(ne_value) = rental_date.ne {
                condition = condition.add(Column::RentalDate.ne(ne_value))
            }
        }
        if let Some(inventory_id) = current_filter.inventory_id {
            if let Some(eq_value) = inventory_id.eq {
                condition = condition.add(Column::InventoryId.eq(eq_value))
            }
            if let Some(ne_value) = inventory_id.ne {
                condition = condition.add(Column::InventoryId.ne(ne_value))
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
        if let Some(return_date) = current_filter.return_date {
            if let Some(eq_value) = return_date.eq {
                condition = condition.add(Column::ReturnDate.eq(eq_value))
            }
            if let Some(ne_value) = return_date.ne {
                condition = condition.add(Column::ReturnDate.ne(ne_value))
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
pub use crate::orm::rental::*;
#[async_graphql::Object(name = "Rental")]
impl Model {
    pub async fn rental_id(&self) -> &i32 {
        &self.rental_id
    }
    pub async fn rental_date(&self) -> &DateTime {
        &self.rental_date
    }
    pub async fn inventory_id(&self) -> &String {
        &self.inventory_id
    }
    pub async fn customer_id(&self) -> &u16 {
        &self.customer_id
    }
    pub async fn return_date(&self) -> &Option<DateTime> {
        &self.return_date
    }
    pub async fn staff_id(&self) -> &u8 {
        &self.staff_id
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
    pub async fn inventory_inventory<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::inventory::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = InventoryInventoryFK(self.inventory_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
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
    pub async fn rental_payment<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::payment::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = RentalPaymentFK(self.rental_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "RentalFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub rental_id: Option<TypeFilter<i32>>,
    pub rental_date: Option<TypeFilter<DateTime>>,
    pub inventory_id: Option<TypeFilter<String>>,
    pub customer_id: Option<TypeFilter<u16>>,
    pub return_date: Option<TypeFilter<DateTime>>,
    pub staff_id: Option<TypeFilter<u8>>,
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
pub struct InventoryInventoryFK(String);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<InventoryInventoryFK> for OrmDataloader {
    type Value = crate::orm::inventory::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[InventoryInventoryFK],
    ) -> Result<std::collections::HashMap<InventoryInventoryFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::inventory::Column::InventoryId.as_column_ref(),
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
        Ok(crate::orm::inventory::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = InventoryInventoryFK(model.inventory_id.clone());
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
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct RentalPaymentFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<RentalPaymentFK> for OrmDataloader {
    type Value = Vec<crate::orm::payment::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[RentalPaymentFK],
    ) -> Result<std::collections::HashMap<RentalPaymentFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::payment::Column::RentalId.as_column_ref(),
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
        use itertools::Itertools;
        Ok(crate::orm::payment::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = RentalPaymentFK(model.rental_id.unwrap().clone());
                (key, model)
            })
            .into_group_map())
    }
}
