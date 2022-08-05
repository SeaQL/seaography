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
        if let Some(country_id) = current_filter.country_id {
            if let Some(eq_value) = country_id.eq {
                condition = condition.add(Column::CountryId.eq(eq_value))
            }
            if let Some(ne_value) = country_id.ne {
                condition = condition.add(Column::CountryId.ne(ne_value))
            }
        }
        if let Some(country) = current_filter.country {
            if let Some(eq_value) = country.eq {
                condition = condition.add(Column::Country.eq(eq_value))
            }
            if let Some(ne_value) = country.ne {
                condition = condition.add(Column::Country.ne(ne_value))
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
pub use crate::orm::country::*;
#[async_graphql::Object(name = "Country")]
impl Model {
    pub async fn country_id(&self) -> &i32 {
        &self.country_id
    }
    pub async fn country(&self) -> &String {
        &self.country
    }
    pub async fn last_update(&self) -> &DateTime {
        &self.last_update
    }
    pub async fn country_city<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::city::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CountryCityFK(self.country_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "CountryFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub country_id: Option<TypeFilter<i32>>,
    pub country: Option<TypeFilter<String>>,
    pub last_update: Option<TypeFilter<DateTime>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CountryCityFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CountryCityFK> for OrmDataloader {
    type Value = Vec<crate::orm::city::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CountryCityFK],
    ) -> Result<std::collections::HashMap<CountryCityFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::city::Column::CountryId.as_column_ref())
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
        Ok(crate::orm::city::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CountryCityFK(model.country_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
