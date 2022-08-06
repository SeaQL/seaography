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
        if let Some(customer_id) = current_filter.customer_id {
            if let Some(eq_value) = customer_id.eq {
                condition = condition.add(Column::CustomerId.eq(eq_value))
            }
            if let Some(ne_value) = customer_id.ne {
                condition = condition.add(Column::CustomerId.ne(ne_value))
            }
        }
        if let Some(store_id) = current_filter.store_id {
            if let Some(eq_value) = store_id.eq {
                condition = condition.add(Column::StoreId.eq(eq_value))
            }
            if let Some(ne_value) = store_id.ne {
                condition = condition.add(Column::StoreId.ne(ne_value))
            }
        }
        if let Some(first_name) = current_filter.first_name {
            if let Some(eq_value) = first_name.eq {
                condition = condition.add(Column::FirstName.eq(eq_value))
            }
            if let Some(ne_value) = first_name.ne {
                condition = condition.add(Column::FirstName.ne(ne_value))
            }
        }
        if let Some(last_name) = current_filter.last_name {
            if let Some(eq_value) = last_name.eq {
                condition = condition.add(Column::LastName.eq(eq_value))
            }
            if let Some(ne_value) = last_name.ne {
                condition = condition.add(Column::LastName.ne(ne_value))
            }
        }
        if let Some(email) = current_filter.email {
            if let Some(eq_value) = email.eq {
                condition = condition.add(Column::Email.eq(eq_value))
            }
            if let Some(ne_value) = email.ne {
                condition = condition.add(Column::Email.ne(ne_value))
            }
        }
        if let Some(address_id) = current_filter.address_id {
            if let Some(eq_value) = address_id.eq {
                condition = condition.add(Column::AddressId.eq(eq_value))
            }
            if let Some(ne_value) = address_id.ne {
                condition = condition.add(Column::AddressId.ne(ne_value))
            }
        }
        if let Some(active) = current_filter.active {
            if let Some(eq_value) = active.eq {
                condition = condition.add(Column::Active.eq(eq_value))
            }
            if let Some(ne_value) = active.ne {
                condition = condition.add(Column::Active.ne(ne_value))
            }
        }
        if let Some(create_date) = current_filter.create_date {
            if let Some(eq_value) = create_date.eq {
                condition = condition.add(Column::CreateDate.eq(eq_value))
            }
            if let Some(ne_value) = create_date.ne {
                condition = condition.add(Column::CreateDate.ne(ne_value))
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
pub use crate::orm::customer::*;
#[async_graphql::Object(name = "Customer")]
impl Model {
    pub async fn customer_id(&self) -> &u16 {
        &self.customer_id
    }
    pub async fn store_id(&self) -> &u8 {
        &self.store_id
    }
    pub async fn first_name(&self) -> &String {
        &self.first_name
    }
    pub async fn last_name(&self) -> &String {
        &self.last_name
    }
    pub async fn email(&self) -> &Option<String> {
        &self.email
    }
    pub async fn address_id(&self) -> &u16 {
        &self.address_id
    }
    pub async fn active(&self) -> &i8 {
        &self.active
    }
    pub async fn create_date(&self) -> &DateTime {
        &self.create_date
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
        &self.last_update
    }
    pub async fn customer_customer_rental<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::rental::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CustomerRentalFK(self.customer_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn customer_address_address<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::address::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = AddressAddressFK(self.address_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn customer_store_store<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::store::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = StoreStoreFK(self.store_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn customer_customer_payment<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::payment::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CustomerPaymentFK(self.customer_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "CustomerFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub customer_id: Option<TypeFilter<u16>>,
    pub store_id: Option<TypeFilter<u8>>,
    pub first_name: Option<TypeFilter<String>>,
    pub last_name: Option<TypeFilter<String>>,
    pub email: Option<TypeFilter<String>>,
    pub address_id: Option<TypeFilter<u16>>,
    pub active: Option<TypeFilter<i8>>,
    pub create_date: Option<TypeFilter<DateTime>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CustomerRentalFK(u16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CustomerRentalFK> for OrmDataloader {
    type Value = Vec<crate::orm::rental::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CustomerRentalFK],
    ) -> Result<std::collections::HashMap<CustomerRentalFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::rental::Column::CustomerId.as_column_ref(),
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
        Ok(crate::orm::rental::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CustomerRentalFK(model.customer_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AddressAddressFK(u16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<AddressAddressFK> for OrmDataloader {
    type Value = crate::orm::address::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[AddressAddressFK],
    ) -> Result<std::collections::HashMap<AddressAddressFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::address::Column::AddressId.as_column_ref(),
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
        Ok(crate::orm::address::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = AddressAddressFK(model.address_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StoreStoreFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StoreStoreFK> for OrmDataloader {
    type Value = crate::orm::store::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StoreStoreFK],
    ) -> Result<std::collections::HashMap<StoreStoreFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::store::Column::StoreId.as_column_ref())
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
        Ok(crate::orm::store::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = StoreStoreFK(model.store_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CustomerPaymentFK(u16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CustomerPaymentFK> for OrmDataloader {
    type Value = Vec<crate::orm::payment::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CustomerPaymentFK],
    ) -> Result<std::collections::HashMap<CustomerPaymentFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::payment::Column::CustomerId.as_column_ref(),
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
                let key = CustomerPaymentFK(model.customer_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
