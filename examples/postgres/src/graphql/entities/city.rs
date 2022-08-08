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
        if let Some(city) = current_filter.city {
            if let Some(eq_value) = city.eq {
                condition = condition.add(Column::City.eq(eq_value))
            }
            if let Some(ne_value) = city.ne {
                condition = condition.add(Column::City.ne(ne_value))
            }
            if let Some(gt_value) = city.gt {
                condition = condition.add(Column::City.gt(gt_value))
            }
            if let Some(gte_value) = city.gte {
                condition = condition.add(Column::City.gte(gte_value))
            }
            if let Some(lt_value) = city.lt {
                condition = condition.add(Column::City.lt(lt_value))
            }
            if let Some(lte_value) = city.lte {
                condition = condition.add(Column::City.lte(lte_value))
            }
            if let Some(is_in_value) = city.is_in {
                condition = condition.add(Column::City.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = city.is_not_in {
                condition = condition.add(Column::City.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = city.is_null {
                if is_null_value {
                    condition = condition.add(Column::City.is_null())
                }
            }
        }
        if let Some(country_id) = current_filter.country_id {
            if let Some(eq_value) = country_id.eq {
                condition = condition.add(Column::CountryId.eq(eq_value))
            }
            if let Some(ne_value) = country_id.ne {
                condition = condition.add(Column::CountryId.ne(ne_value))
            }
            if let Some(gt_value) = country_id.gt {
                condition = condition.add(Column::CountryId.gt(gt_value))
            }
            if let Some(gte_value) = country_id.gte {
                condition = condition.add(Column::CountryId.gte(gte_value))
            }
            if let Some(lt_value) = country_id.lt {
                condition = condition.add(Column::CountryId.lt(lt_value))
            }
            if let Some(lte_value) = country_id.lte {
                condition = condition.add(Column::CountryId.lte(lte_value))
            }
            if let Some(is_in_value) = country_id.is_in {
                condition = condition.add(Column::CountryId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = country_id.is_not_in {
                condition = condition.add(Column::CountryId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = country_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::CountryId.is_null())
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
pub use crate::orm::city::*;
#[async_graphql::Object(name = "City")]
impl Model {
    pub async fn city_id(&self) -> &i32 {
        &self.city_id
    }
    pub async fn city(&self) -> &String {
        &self.city
    }
    pub async fn country_id(&self) -> &i16 {
        &self.country_id
    }
    pub async fn last_update(&self) -> &DateTime {
        &self.last_update
    }
    pub async fn city_country_country<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::country::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CountryCountryFK(self.country_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn city_city_address<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::address::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CityAddressFK(self.city_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "CityFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub city_id: Option<TypeFilter<i32>>,
    pub city: Option<TypeFilter<String>>,
    pub country_id: Option<TypeFilter<i16>>,
    pub last_update: Option<TypeFilter<DateTime>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CountryCountryFK(i16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CountryCountryFK> for OrmDataloader {
    type Value = crate::orm::country::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CountryCountryFK],
    ) -> Result<std::collections::HashMap<CountryCountryFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::country::Column::CountryId.as_column_ref(),
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
        Ok(crate::orm::country::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CountryCountryFK(model.country_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CityAddressFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CityAddressFK> for OrmDataloader {
    type Value = Vec<crate::orm::address::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CityAddressFK],
    ) -> Result<std::collections::HashMap<CityAddressFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::address::Column::CityId.as_column_ref())
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
        Ok(crate::orm::address::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CityAddressFK(model.city_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
