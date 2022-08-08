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
        if let Some(title) = current_filter.title {
            if let Some(eq_value) = title.eq {
                condition = condition.add(Column::Title.eq(eq_value))
            }
            if let Some(ne_value) = title.ne {
                condition = condition.add(Column::Title.ne(ne_value))
            }
            if let Some(gt_value) = title.gt {
                condition = condition.add(Column::Title.gt(gt_value))
            }
            if let Some(gte_value) = title.gte {
                condition = condition.add(Column::Title.gte(gte_value))
            }
            if let Some(lt_value) = title.lt {
                condition = condition.add(Column::Title.lt(lt_value))
            }
            if let Some(lte_value) = title.lte {
                condition = condition.add(Column::Title.lte(lte_value))
            }
            if let Some(is_in_value) = title.is_in {
                condition = condition.add(Column::Title.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = title.is_not_in {
                condition = condition.add(Column::Title.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = title.is_null {
                if is_null_value {
                    condition = condition.add(Column::Title.is_null())
                }
            }
        }
        if let Some(description) = current_filter.description {
            if let Some(eq_value) = description.eq {
                condition = condition.add(Column::Description.eq(eq_value))
            }
            if let Some(ne_value) = description.ne {
                condition = condition.add(Column::Description.ne(ne_value))
            }
            if let Some(gt_value) = description.gt {
                condition = condition.add(Column::Description.gt(gt_value))
            }
            if let Some(gte_value) = description.gte {
                condition = condition.add(Column::Description.gte(gte_value))
            }
            if let Some(lt_value) = description.lt {
                condition = condition.add(Column::Description.lt(lt_value))
            }
            if let Some(lte_value) = description.lte {
                condition = condition.add(Column::Description.lte(lte_value))
            }
            if let Some(is_in_value) = description.is_in {
                condition = condition.add(Column::Description.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = description.is_not_in {
                condition = condition.add(Column::Description.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = description.is_null {
                if is_null_value {
                    condition = condition.add(Column::Description.is_null())
                }
            }
        }
    }
    condition
}
use crate::graphql::*;
pub use crate::orm::film_text::*;
#[async_graphql::Object(name = "FilmText")]
impl Model {
    pub async fn film_id(&self) -> &i16 {
        &self.film_id
    }
    pub async fn title(&self) -> &String {
        &self.title
    }
    pub async fn description(&self) -> &Option<String> {
        &self.description
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "FilmTextFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub film_id: Option<TypeFilter<i16>>,
    pub title: Option<TypeFilter<String>>,
    pub description: Option<TypeFilter<String>>,
}
