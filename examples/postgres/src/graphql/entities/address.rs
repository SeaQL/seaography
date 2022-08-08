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
            if let Some(gt_value) = address_id.gt {
                condition = condition.add(Column::AddressId.gt(gt_value))
            }
            if let Some(gte_value) = address_id.gte {
                condition = condition.add(Column::AddressId.gte(gte_value))
            }
            if let Some(lt_value) = address_id.lt {
                condition = condition.add(Column::AddressId.lt(lt_value))
            }
            if let Some(lte_value) = address_id.lte {
                condition = condition.add(Column::AddressId.lte(lte_value))
            }
            if let Some(is_in_value) = address_id.is_in {
                condition = condition.add(Column::AddressId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = address_id.is_not_in {
                condition = condition.add(Column::AddressId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = address_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::AddressId.is_null())
                }
            }
        }
        if let Some(address) = current_filter.address {
            if let Some(eq_value) = address.eq {
                condition = condition.add(Column::Address.eq(eq_value))
            }
            if let Some(ne_value) = address.ne {
                condition = condition.add(Column::Address.ne(ne_value))
            }
            if let Some(gt_value) = address.gt {
                condition = condition.add(Column::Address.gt(gt_value))
            }
            if let Some(gte_value) = address.gte {
                condition = condition.add(Column::Address.gte(gte_value))
            }
            if let Some(lt_value) = address.lt {
                condition = condition.add(Column::Address.lt(lt_value))
            }
            if let Some(lte_value) = address.lte {
                condition = condition.add(Column::Address.lte(lte_value))
            }
            if let Some(is_in_value) = address.is_in {
                condition = condition.add(Column::Address.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = address.is_not_in {
                condition = condition.add(Column::Address.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = address.is_null {
                if is_null_value {
                    condition = condition.add(Column::Address.is_null())
                }
            }
        }
        if let Some(address2) = current_filter.address2 {
            if let Some(eq_value) = address2.eq {
                condition = condition.add(Column::Address2.eq(eq_value))
            }
            if let Some(ne_value) = address2.ne {
                condition = condition.add(Column::Address2.ne(ne_value))
            }
            if let Some(gt_value) = address2.gt {
                condition = condition.add(Column::Address2.gt(gt_value))
            }
            if let Some(gte_value) = address2.gte {
                condition = condition.add(Column::Address2.gte(gte_value))
            }
            if let Some(lt_value) = address2.lt {
                condition = condition.add(Column::Address2.lt(lt_value))
            }
            if let Some(lte_value) = address2.lte {
                condition = condition.add(Column::Address2.lte(lte_value))
            }
            if let Some(is_in_value) = address2.is_in {
                condition = condition.add(Column::Address2.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = address2.is_not_in {
                condition = condition.add(Column::Address2.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = address2.is_null {
                if is_null_value {
                    condition = condition.add(Column::Address2.is_null())
                }
            }
        }
        if let Some(district) = current_filter.district {
            if let Some(eq_value) = district.eq {
                condition = condition.add(Column::District.eq(eq_value))
            }
            if let Some(ne_value) = district.ne {
                condition = condition.add(Column::District.ne(ne_value))
            }
            if let Some(gt_value) = district.gt {
                condition = condition.add(Column::District.gt(gt_value))
            }
            if let Some(gte_value) = district.gte {
                condition = condition.add(Column::District.gte(gte_value))
            }
            if let Some(lt_value) = district.lt {
                condition = condition.add(Column::District.lt(lt_value))
            }
            if let Some(lte_value) = district.lte {
                condition = condition.add(Column::District.lte(lte_value))
            }
            if let Some(is_in_value) = district.is_in {
                condition = condition.add(Column::District.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = district.is_not_in {
                condition = condition.add(Column::District.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = district.is_null {
                if is_null_value {
                    condition = condition.add(Column::District.is_null())
                }
            }
        }
        if let Some(city_id) = current_filter.city_id {
            if let Some(eq_value) = city_id.eq {
                condition = condition.add(Column::CityId.eq(eq_value))
            }
            if let Some(ne_value) = city_id.ne {
                condition = condition.add(Column::CityId.ne(ne_value))
            }
            if let Some(gt_value) = city_id.gt {
                condition = condition.add(Column::CityId.gt(gt_value))
            }
            if let Some(gte_value) = city_id.gte {
                condition = condition.add(Column::CityId.gte(gte_value))
            }
            if let Some(lt_value) = city_id.lt {
                condition = condition.add(Column::CityId.lt(lt_value))
            }
            if let Some(lte_value) = city_id.lte {
                condition = condition.add(Column::CityId.lte(lte_value))
            }
            if let Some(is_in_value) = city_id.is_in {
                condition = condition.add(Column::CityId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = city_id.is_not_in {
                condition = condition.add(Column::CityId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = city_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::CityId.is_null())
                }
            }
        }
        if let Some(postal_code) = current_filter.postal_code {
            if let Some(eq_value) = postal_code.eq {
                condition = condition.add(Column::PostalCode.eq(eq_value))
            }
            if let Some(ne_value) = postal_code.ne {
                condition = condition.add(Column::PostalCode.ne(ne_value))
            }
            if let Some(gt_value) = postal_code.gt {
                condition = condition.add(Column::PostalCode.gt(gt_value))
            }
            if let Some(gte_value) = postal_code.gte {
                condition = condition.add(Column::PostalCode.gte(gte_value))
            }
            if let Some(lt_value) = postal_code.lt {
                condition = condition.add(Column::PostalCode.lt(lt_value))
            }
            if let Some(lte_value) = postal_code.lte {
                condition = condition.add(Column::PostalCode.lte(lte_value))
            }
            if let Some(is_in_value) = postal_code.is_in {
                condition = condition.add(Column::PostalCode.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = postal_code.is_not_in {
                condition = condition.add(Column::PostalCode.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = postal_code.is_null {
                if is_null_value {
                    condition = condition.add(Column::PostalCode.is_null())
                }
            }
        }
        if let Some(phone) = current_filter.phone {
            if let Some(eq_value) = phone.eq {
                condition = condition.add(Column::Phone.eq(eq_value))
            }
            if let Some(ne_value) = phone.ne {
                condition = condition.add(Column::Phone.ne(ne_value))
            }
            if let Some(gt_value) = phone.gt {
                condition = condition.add(Column::Phone.gt(gt_value))
            }
            if let Some(gte_value) = phone.gte {
                condition = condition.add(Column::Phone.gte(gte_value))
            }
            if let Some(lt_value) = phone.lt {
                condition = condition.add(Column::Phone.lt(lt_value))
            }
            if let Some(lte_value) = phone.lte {
                condition = condition.add(Column::Phone.lte(lte_value))
            }
            if let Some(is_in_value) = phone.is_in {
                condition = condition.add(Column::Phone.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = phone.is_not_in {
                condition = condition.add(Column::Phone.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = phone.is_null {
                if is_null_value {
                    condition = condition.add(Column::Phone.is_null())
                }
            }
        }
        if let Some(last_update) = current_filter.last_update {
            if let Some(eq_value) = last_update.eq {
                condition = condition.add(Column::LastUpdate.eq(eq_value))
            }
            if let Some(ne_value) = last_update.ne {
                condition = condition.add(Column::LastUpdate.ne(ne_value))
            }
            if let Some(gt_value) = last_update.gt {
                condition = condition.add(Column::LastUpdate.gt(gt_value))
            }
            if let Some(gte_value) = last_update.gte {
                condition = condition.add(Column::LastUpdate.gte(gte_value))
            }
            if let Some(lt_value) = last_update.lt {
                condition = condition.add(Column::LastUpdate.lt(lt_value))
            }
            if let Some(lte_value) = last_update.lte {
                condition = condition.add(Column::LastUpdate.lte(lte_value))
            }
            if let Some(is_in_value) = last_update.is_in {
                condition = condition.add(Column::LastUpdate.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = last_update.is_not_in {
                condition = condition.add(Column::LastUpdate.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = last_update.is_null {
                if is_null_value {
                    condition = condition.add(Column::LastUpdate.is_null())
                }
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
    pub async fn address_address_staff<'a>(
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
    pub async fn address_address_store<'a>(
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
    pub async fn address_address_customer<'a>(
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
    pub async fn address_city_city<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::city::Model {
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
