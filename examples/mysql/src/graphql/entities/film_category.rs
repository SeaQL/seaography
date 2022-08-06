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
        }
        if let Some(category_id) = current_filter.category_id {
            if let Some(eq_value) = category_id.eq {
                condition = condition.add(Column::CategoryId.eq(eq_value))
            }
            if let Some(ne_value) = category_id.ne {
                condition = condition.add(Column::CategoryId.ne(ne_value))
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
pub use crate::orm::film_category::*;
#[async_graphql::Object(name = "FilmCategory")]
impl Model {
    pub async fn film_id(&self) -> &u16 {
        &self.film_id
    }
    pub async fn category_id(&self) -> &u8 {
        &self.category_id
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
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
    pub film_id: Option<TypeFilter<u16>>,
    pub category_id: Option<TypeFilter<u8>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CategoryCategoryFK(u8);
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
pub struct FilmFilmFK(u16);
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
