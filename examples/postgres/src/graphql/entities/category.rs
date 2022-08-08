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
        if let Some(name) = current_filter.name {
            if let Some(eq_value) = name.eq {
                condition = condition.add(Column::Name.eq(eq_value))
            }
            if let Some(ne_value) = name.ne {
                condition = condition.add(Column::Name.ne(ne_value))
            }
            if let Some(gt_value) = name.gt {
                condition = condition.add(Column::Name.gt(gt_value))
            }
            if let Some(gte_value) = name.gte {
                condition = condition.add(Column::Name.gte(gte_value))
            }
            if let Some(lt_value) = name.lt {
                condition = condition.add(Column::Name.lt(lt_value))
            }
            if let Some(lte_value) = name.lte {
                condition = condition.add(Column::Name.lte(lte_value))
            }
            if let Some(is_in_value) = name.is_in {
                condition = condition.add(Column::Name.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = name.is_not_in {
                condition = condition.add(Column::Name.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = name.is_null {
                if is_null_value {
                    condition = condition.add(Column::Name.is_null())
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
pub use crate::orm::category::*;
#[async_graphql::Object(name = "Category")]
impl Model {
    pub async fn category_id(&self) -> &i32 {
        &self.category_id
    }
    pub async fn name(&self) -> &String {
        &self.name
    }
    pub async fn last_update(&self) -> &DateTime {
        &self.last_update
    }
    pub async fn category_category_film_category<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::film_category::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CategoryFilmCategoryFK(self.category_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "CategoryFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub category_id: Option<TypeFilter<i32>>,
    pub name: Option<TypeFilter<String>>,
    pub last_update: Option<TypeFilter<DateTime>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CategoryFilmCategoryFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CategoryFilmCategoryFK> for OrmDataloader {
    type Value = Vec<crate::orm::film_category::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CategoryFilmCategoryFK],
    ) -> Result<std::collections::HashMap<CategoryFilmCategoryFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::film_category::Column::CategoryId.as_column_ref(),
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
        Ok(crate::orm::film_category::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CategoryFilmCategoryFK(model.category_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
