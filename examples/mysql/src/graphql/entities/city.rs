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
        }
        if let Some(city) = current_filter.city {
            if let Some(eq_value) = city.eq {
                condition = condition.add(Column::City.eq(eq_value))
            }
            if let Some(ne_value) = city.ne {
                condition = condition.add(Column::City.ne(ne_value))
            }
        }
        if let Some(country_id) = current_filter.country_id {
            if let Some(eq_value) = country_id.eq {
                condition = condition.add(Column::CountryId.eq(eq_value))
            }
            if let Some(ne_value) = country_id.ne {
                condition = condition.add(Column::CountryId.ne(ne_value))
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
pub use crate::orm::city::*;
#[async_graphql::Object(name = "City")]
impl Model {
    pub async fn city_id(&self) -> &u16 {
        &self.city_id
    }
    pub async fn city(&self) -> &String {
        &self.city
    }
    pub async fn country_id(&self) -> &u16 {
        &self.country_id
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
        &self.last_update
    }
    pub async fn city_address<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::address::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CityAddressFK(self.city_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn country_country<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::country::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CountryCountryFK(self.country_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "CityFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub city_id: Option<TypeFilter<u16>>,
    pub city: Option<TypeFilter<String>>,
    pub country_id: Option<TypeFilter<u16>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CityAddressFK(u16);
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
                let key = CityAddressFK(model.city_id.clone());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CountryCountryFK(u16);
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
                let key = CountryCountryFK(model.country_id.clone());
                (key, model)
            })
            .collect())
    }
}
