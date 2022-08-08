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
        if let Some(film_id) = current_filter.film_id {
            if let Some(eq_value) = film_id.eq {
                condition = condition.add(Column::FilmId.eq(eq_value))
            }
            if let Some(ne_value) = film_id.ne {
                condition = condition.add(Column::FilmId.ne(ne_value))
            }
            if let Some(gt_value) = film_id.gt {
                condition = condition.add(Column::FilmId.gt(gt_value))
            }
            if let Some(gte_value) = film_id.gte {
                condition = condition.add(Column::FilmId.gte(gte_value))
            }
            if let Some(lt_value) = film_id.lt {
                condition = condition.add(Column::FilmId.lt(lt_value))
            }
            if let Some(lte_value) = film_id.lte {
                condition = condition.add(Column::FilmId.lte(lte_value))
            }
            if let Some(is_in_value) = film_id.is_in {
                condition = condition.add(Column::FilmId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = film_id.is_not_in {
                condition = condition.add(Column::FilmId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = film_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::FilmId.is_null())
                }
            }
        }
        if let Some(category_id) = current_filter.category_id {
            if let Some(eq_value) = category_id.eq {
                condition = condition.add(Column::CategoryId.eq(eq_value))
            }
            if let Some(ne_value) = category_id.ne {
                condition = condition.add(Column::CategoryId.ne(ne_value))
            }
            if let Some(gt_value) = category_id.gt {
                condition = condition.add(Column::CategoryId.gt(gt_value))
            }
            if let Some(gte_value) = category_id.gte {
                condition = condition.add(Column::CategoryId.gte(gte_value))
            }
            if let Some(lt_value) = category_id.lt {
                condition = condition.add(Column::CategoryId.lt(lt_value))
            }
            if let Some(lte_value) = category_id.lte {
                condition = condition.add(Column::CategoryId.lte(lte_value))
            }
            if let Some(is_in_value) = category_id.is_in {
                condition = condition.add(Column::CategoryId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = category_id.is_not_in {
                condition = condition.add(Column::CategoryId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = category_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::CategoryId.is_null())
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
pub use crate::orm::film_category::*;
#[async_graphql::Object(name = "FilmCategory")]
impl Model {
    pub async fn film_id(&self) -> &i16 {
        &self.film_id
    }
    pub async fn category_id(&self) -> &i16 {
        &self.category_id
    }
    pub async fn last_update(&self) -> &DateTime {
        &self.last_update
    }
    pub async fn film_category_category_category<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::category::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CategoryCategoryFK(self.category_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn film_category_film_film<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::film::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = FilmFilmFK(self.film_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "FilmCategoryFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub film_id: Option<TypeFilter<i16>>,
    pub category_id: Option<TypeFilter<i16>>,
    pub last_update: Option<TypeFilter<DateTime>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CategoryCategoryFK(i16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CategoryCategoryFK> for OrmDataloader {
    type Value = crate::orm::category::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CategoryCategoryFK],
    ) -> Result<std::collections::HashMap<CategoryCategoryFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::category::Column::CategoryId.as_column_ref(),
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
        Ok(crate::orm::category::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CategoryCategoryFK(model.category_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FilmFilmFK(i16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<FilmFilmFK> for OrmDataloader {
    type Value = crate::orm::film::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[FilmFilmFK],
    ) -> Result<std::collections::HashMap<FilmFilmFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::film::Column::FilmId.as_column_ref())
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
        Ok(crate::orm::film::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = FilmFilmFK(model.film_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
