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
        if let Some(title) = current_filter.title {
            if let Some(eq_value) = title.eq {
                condition = condition.add(Column::Title.eq(eq_value))
            }
            if let Some(ne_value) = title.ne {
                condition = condition.add(Column::Title.ne(ne_value))
            }
        }
        if let Some(description) = current_filter.description {
            if let Some(eq_value) = description.eq {
                condition = condition.add(Column::Description.eq(eq_value))
            }
            if let Some(ne_value) = description.ne {
                condition = condition.add(Column::Description.ne(ne_value))
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
