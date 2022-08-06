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
        if let Some(inventory_id) = current_filter.inventory_id {
            if let Some(eq_value) = inventory_id.eq {
                condition = condition.add(Column::InventoryId.eq(eq_value))
            }
            if let Some(ne_value) = inventory_id.ne {
                condition = condition.add(Column::InventoryId.ne(ne_value))
            }
        }
        if let Some(film_id) = current_filter.film_id {
            if let Some(eq_value) = film_id.eq {
                condition = condition.add(Column::FilmId.eq(eq_value))
            }
            if let Some(ne_value) = film_id.ne {
                condition = condition.add(Column::FilmId.ne(ne_value))
            }
        }
        if let Some(store_id) = current_filter.store_id {
            if let Some(eq_value) = store_id.eq {
                condition = condition.add(Column::StoreId.eq(eq_value))
            }
            if let Some(ne_value) = store_id.ne {
                condition = condition.add(Column::StoreId.ne(ne_value))
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
pub use crate::orm::inventory::*;
#[async_graphql::Object(name = "Inventory")]
impl Model {
    pub async fn inventory_id(&self) -> &String {
        &self.inventory_id
    }
    pub async fn film_id(&self) -> &u16 {
        &self.film_id
    }
    pub async fn store_id(&self) -> &u8 {
        &self.store_id
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
        &self.last_update
    }
    pub async fn inventory_film_film<'a>(
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
    pub async fn inventory_store_store<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::store::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = StoreStoreFK(self.store_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn inventory_inventory_rental<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::rental::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = InventoryRentalFK(self.inventory_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "InventoryFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub inventory_id: Option<TypeFilter<String>>,
    pub film_id: Option<TypeFilter<u16>>,
    pub store_id: Option<TypeFilter<u8>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
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
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StoreStoreFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StoreStoreFK> for OrmDataloader {
    type Value = crate::orm::store::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StoreStoreFK],
    ) -> Result<std::collections::HashMap<StoreStoreFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::store::Column::StoreId.as_column_ref())
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
        Ok(crate::orm::store::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = StoreStoreFK(model.store_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct InventoryRentalFK(String);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<InventoryRentalFK> for OrmDataloader {
    type Value = Vec<crate::orm::rental::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[InventoryRentalFK],
    ) -> Result<std::collections::HashMap<InventoryRentalFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::rental::Column::InventoryId.as_column_ref(),
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
        Ok(crate::orm::rental::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = InventoryRentalFK(model.inventory_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
