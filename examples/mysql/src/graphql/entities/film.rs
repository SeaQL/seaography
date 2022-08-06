use crate::graphql::enums::Rating;
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
        if let Some(release_year) = current_filter.release_year {
            if let Some(eq_value) = release_year.eq {
                condition = condition.add(Column::ReleaseYear.eq(eq_value))
            }
            if let Some(ne_value) = release_year.ne {
                condition = condition.add(Column::ReleaseYear.ne(ne_value))
            }
        }
        if let Some(language_id) = current_filter.language_id {
            if let Some(eq_value) = language_id.eq {
                condition = condition.add(Column::LanguageId.eq(eq_value))
            }
            if let Some(ne_value) = language_id.ne {
                condition = condition.add(Column::LanguageId.ne(ne_value))
            }
        }
        if let Some(original_language_id) = current_filter.original_language_id {
            if let Some(eq_value) = original_language_id.eq {
                condition = condition.add(Column::OriginalLanguageId.eq(eq_value))
            }
            if let Some(ne_value) = original_language_id.ne {
                condition = condition.add(Column::OriginalLanguageId.ne(ne_value))
            }
        }
        if let Some(rental_duration) = current_filter.rental_duration {
            if let Some(eq_value) = rental_duration.eq {
                condition = condition.add(Column::RentalDuration.eq(eq_value))
            }
            if let Some(ne_value) = rental_duration.ne {
                condition = condition.add(Column::RentalDuration.ne(ne_value))
            }
        }
        if let Some(rental_rate) = current_filter.rental_rate {
            if let Some(eq_value) = rental_rate.eq {
                condition = condition.add(Column::RentalRate.eq(eq_value))
            }
            if let Some(ne_value) = rental_rate.ne {
                condition = condition.add(Column::RentalRate.ne(ne_value))
            }
        }
        if let Some(length) = current_filter.length {
            if let Some(eq_value) = length.eq {
                condition = condition.add(Column::Length.eq(eq_value))
            }
            if let Some(ne_value) = length.ne {
                condition = condition.add(Column::Length.ne(ne_value))
            }
        }
        if let Some(replacement_cost) = current_filter.replacement_cost {
            if let Some(eq_value) = replacement_cost.eq {
                condition = condition.add(Column::ReplacementCost.eq(eq_value))
            }
            if let Some(ne_value) = replacement_cost.ne {
                condition = condition.add(Column::ReplacementCost.ne(ne_value))
            }
        }
        if let Some(special_features) = current_filter.special_features {
            if let Some(eq_value) = special_features.eq {
                condition = condition.add(Column::SpecialFeatures.eq(eq_value))
            }
            if let Some(ne_value) = special_features.ne {
                condition = condition.add(Column::SpecialFeatures.ne(ne_value))
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
pub use crate::orm::film::*;
#[async_graphql::Object(name = "Film")]
impl Model {
    pub async fn film_id(&self) -> &u16 {
        &self.film_id
    }
    pub async fn title(&self) -> &String {
        &self.title
    }
    pub async fn description(&self) -> &Option<String> {
        &self.description
    }
    pub async fn release_year(&self) -> &Option<String> {
        &self.release_year
    }
    pub async fn language_id(&self) -> &u8 {
        &self.language_id
    }
    pub async fn original_language_id(&self) -> &Option<u8> {
        &self.original_language_id
    }
    pub async fn rental_duration(&self) -> &u8 {
        &self.rental_duration
    }
    pub async fn rental_rate(&self) -> &Decimal {
        &self.rental_rate
    }
    pub async fn length(&self) -> &Option<u16> {
        &self.length
    }
    pub async fn replacement_cost(&self) -> &Decimal {
        &self.replacement_cost
    }
    pub async fn rating(&self) -> Option<Rating> {
        self.rating.clone().map(|i| i.into())
    }
    pub async fn special_features(&self) -> &Option<String> {
        &self.special_features
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
        &self.last_update
    }
    pub async fn film_film_inventory<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::inventory::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = FilmInventoryFK(self.film_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn film_film_film_category<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::film_category::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = FilmFilmCategoryFK(self.film_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn film_language_language<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::language::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = LanguageLanguageFK(self.language_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn film_original_language_language<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<crate::orm::language::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = OriginalLanguageLanguageFK(self.original_language_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn film_film_film_actor<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::film_actor::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = FilmFilmActorFK(self.film_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "FilmFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub film_id: Option<TypeFilter<u16>>,
    pub title: Option<TypeFilter<String>>,
    pub description: Option<TypeFilter<String>>,
    pub release_year: Option<TypeFilter<String>>,
    pub language_id: Option<TypeFilter<u8>>,
    pub original_language_id: Option<TypeFilter<u8>>,
    pub rental_duration: Option<TypeFilter<u8>>,
    pub rental_rate: Option<TypeFilter<Decimal>>,
    pub length: Option<TypeFilter<u16>>,
    pub replacement_cost: Option<TypeFilter<Decimal>>,
    pub special_features: Option<TypeFilter<String>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FilmInventoryFK(u16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<FilmInventoryFK> for OrmDataloader {
    type Value = Vec<crate::orm::inventory::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[FilmInventoryFK],
    ) -> Result<std::collections::HashMap<FilmInventoryFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::inventory::Column::FilmId.as_column_ref(),
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
        Ok(crate::orm::inventory::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = FilmInventoryFK(model.film_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FilmFilmCategoryFK(u16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<FilmFilmCategoryFK> for OrmDataloader {
    type Value = Vec<crate::orm::film_category::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[FilmFilmCategoryFK],
    ) -> Result<std::collections::HashMap<FilmFilmCategoryFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::film_category::Column::FilmId.as_column_ref(),
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
                let key = FilmFilmCategoryFK(model.film_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageLanguageFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<LanguageLanguageFK> for OrmDataloader {
    type Value = crate::orm::language::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[LanguageLanguageFK],
    ) -> Result<std::collections::HashMap<LanguageLanguageFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::language::Column::LanguageId.as_column_ref(),
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
        Ok(crate::orm::language::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = LanguageLanguageFK(model.language_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct OriginalLanguageLanguageFK(Option<u8>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<OriginalLanguageLanguageFK> for OrmDataloader {
    type Value = crate::orm::language::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[OriginalLanguageLanguageFK],
    ) -> Result<std::collections::HashMap<OriginalLanguageLanguageFK, Self::Value>, Self::Error>
    {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::language::Column::LanguageId.as_column_ref(),
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
        Ok(crate::orm::language::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = OriginalLanguageLanguageFK(
                    Some(model.language_id.clone()).clone().try_into().unwrap(),
                );
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FilmFilmActorFK(u16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<FilmFilmActorFK> for OrmDataloader {
    type Value = Vec<crate::orm::film_actor::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[FilmFilmActorFK],
    ) -> Result<std::collections::HashMap<FilmFilmActorFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::film_actor::Column::FilmId.as_column_ref(),
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
        Ok(crate::orm::film_actor::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = FilmFilmActorFK(model.film_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
