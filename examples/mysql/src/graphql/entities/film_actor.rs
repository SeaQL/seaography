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
        if let Some(actor_id) = current_filter.actor_id {
            if let Some(eq_value) = actor_id.eq {
                condition = condition.add(Column::ActorId.eq(eq_value))
            }
            if let Some(ne_value) = actor_id.ne {
                condition = condition.add(Column::ActorId.ne(ne_value))
            }
            if let Some(gt_value) = actor_id.gt {
                condition = condition.add(Column::ActorId.gt(gt_value))
            }
            if let Some(gte_value) = actor_id.gte {
                condition = condition.add(Column::ActorId.gte(gte_value))
            }
            if let Some(lt_value) = actor_id.lt {
                condition = condition.add(Column::ActorId.lt(lt_value))
            }
            if let Some(lte_value) = actor_id.lte {
                condition = condition.add(Column::ActorId.lte(lte_value))
            }
            if let Some(is_in_value) = actor_id.is_in {
                condition = condition.add(Column::ActorId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = actor_id.is_not_in {
                condition = condition.add(Column::ActorId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = actor_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::ActorId.is_null())
                }
            }
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
pub use crate::orm::film_actor::*;
#[async_graphql::Object(name = "FilmActor")]
impl Model {
    pub async fn actor_id(&self) -> &u16 {
        &self.actor_id
    }
    pub async fn film_id(&self) -> &u16 {
        &self.film_id
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
        &self.last_update
    }
    pub async fn film_actor_actor_actor<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::actor::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = ActorActorFK(self.actor_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn film_actor_film_film<'a>(
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
#[graphql(name = "FilmActorFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub actor_id: Option<TypeFilter<u16>>,
    pub film_id: Option<TypeFilter<u16>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ActorActorFK(u16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<ActorActorFK> for OrmDataloader {
    type Value = crate::orm::actor::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[ActorActorFK],
    ) -> Result<std::collections::HashMap<ActorActorFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::actor::Column::ActorId.as_column_ref())
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
        Ok(crate::orm::actor::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = ActorActorFK(model.actor_id.clone().try_into().unwrap());
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
