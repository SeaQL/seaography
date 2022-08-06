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
        if let Some(address_id) = current_filter.address_id {
            if let Some(eq_value) = address_id.eq {
                condition = condition.add(Column::AddressId.eq(eq_value))
            }
            if let Some(ne_value) = address_id.ne {
                condition = condition.add(Column::AddressId.ne(ne_value))
            }
        }
        if let Some(address) = current_filter.address {
            if let Some(eq_value) = address.eq {
                condition = condition.add(Column::Address.eq(eq_value))
            }
            if let Some(ne_value) = address.ne {
                condition = condition.add(Column::Address.ne(ne_value))
            }
        }
        if let Some(address2) = current_filter.address2 {
            if let Some(eq_value) = address2.eq {
                condition = condition.add(Column::Address2.eq(eq_value))
            }
            if let Some(ne_value) = address2.ne {
                condition = condition.add(Column::Address2.ne(ne_value))
            }
        }
        if let Some(district) = current_filter.district {
            if let Some(eq_value) = district.eq {
                condition = condition.add(Column::District.eq(eq_value))
            }
            if let Some(ne_value) = district.ne {
                condition = condition.add(Column::District.ne(ne_value))
            }
        }
        if let Some(city_id) = current_filter.city_id {
            if let Some(eq_value) = city_id.eq {
                condition = condition.add(Column::CityId.eq(eq_value))
            }
            if let Some(ne_value) = city_id.ne {
                condition = condition.add(Column::CityId.ne(ne_value))
            }
        }
        if let Some(postal_code) = current_filter.postal_code {
            if let Some(eq_value) = postal_code.eq {
                condition = condition.add(Column::PostalCode.eq(eq_value))
            }
            if let Some(ne_value) = postal_code.ne {
                condition = condition.add(Column::PostalCode.ne(ne_value))
            }
        }
        if let Some(phone) = current_filter.phone {
            if let Some(eq_value) = phone.eq {
                condition = condition.add(Column::Phone.eq(eq_value))
            }
            if let Some(ne_value) = phone.ne {
                condition = condition.add(Column::Phone.ne(ne_value))
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
pub use crate::orm::address::*;
#[async_graphql::Object(name = "Address")]
impl Model {
    pub async fn address_id(&self) -> &i32 {
        &self.address_id
    }
    pub async fn address(&self) -> &String {
        &self.address
    }
    pub async fn address2(&self) -> &Option<String> {
        &self.address2
    }
    pub async fn district(&self) -> &String {
        &self.district
    }
    pub async fn city_id(&self) -> &i16 {
        &self.city_id
    }
    pub async fn postal_code(&self) -> &Option<String> {
        &self.postal_code
    }
    pub async fn phone(&self) -> &String {
        &self.phone
    }
    pub async fn last_update(&self) -> &DateTime {
        &self.last_update
    }
    pub async fn address_store<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::store::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = AddressStoreFK(self.address_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn address_staff<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::staff::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = AddressStaffFK(self.address_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn address_customer<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::customer::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = AddressCustomerFK(self.address_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn city_city<'a>(&self, ctx: &async_graphql::Context<'a>) -> crate::orm::city::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CityCityFK(self.city_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "AddressFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub address_id: Option<TypeFilter<i32>>,
    pub address: Option<TypeFilter<String>>,
    pub address2: Option<TypeFilter<String>>,
    pub district: Option<TypeFilter<String>>,
    pub city_id: Option<TypeFilter<i16>>,
    pub postal_code: Option<TypeFilter<String>>,
    pub phone: Option<TypeFilter<String>>,
    pub last_update: Option<TypeFilter<DateTime>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AddressStoreFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<AddressStoreFK> for OrmDataloader {
    type Value = Vec<crate::orm::store::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[AddressStoreFK],
    ) -> Result<std::collections::HashMap<AddressStoreFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::store::Column::AddressId.as_column_ref())
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
        Ok(crate::orm::store::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = AddressStoreFK(model.address_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AddressStaffFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<AddressStaffFK> for OrmDataloader {
    type Value = Vec<crate::orm::staff::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[AddressStaffFK],
    ) -> Result<std::collections::HashMap<AddressStaffFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::staff::Column::AddressId.as_column_ref())
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
        Ok(crate::orm::staff::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = AddressStaffFK(model.address_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AddressCustomerFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<AddressCustomerFK> for OrmDataloader {
    type Value = Vec<crate::orm::customer::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[AddressCustomerFK],
    ) -> Result<std::collections::HashMap<AddressCustomerFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::customer::Column::AddressId.as_column_ref(),
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
        Ok(crate::orm::customer::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = AddressCustomerFK(model.address_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CityCityFK(i16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CityCityFK> for OrmDataloader {
    type Value = crate::orm::city::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CityCityFK],
    ) -> Result<std::collections::HashMap<CityCityFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::city::Column::CityId.as_column_ref())
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
        Ok(crate::orm::city::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CityCityFK(model.city_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
