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
        if let Some(store_id) = current_filter.store_id {
            if let Some(eq_value) = store_id.eq {
                condition = condition.add(Column::StoreId.eq(eq_value))
            }
            if let Some(ne_value) = store_id.ne {
                condition = condition.add(Column::StoreId.ne(ne_value))
            }
        }
        if let Some(manager_staff_id) = current_filter.manager_staff_id {
            if let Some(eq_value) = manager_staff_id.eq {
                condition = condition.add(Column::ManagerStaffId.eq(eq_value))
            }
            if let Some(ne_value) = manager_staff_id.ne {
                condition = condition.add(Column::ManagerStaffId.ne(ne_value))
            }
        }
        if let Some(address_id) = current_filter.address_id {
            if let Some(eq_value) = address_id.eq {
                condition = condition.add(Column::AddressId.eq(eq_value))
            }
            if let Some(ne_value) = address_id.ne {
                condition = condition.add(Column::AddressId.ne(ne_value))
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
pub use crate::orm::store::*;
#[async_graphql::Object(name = "Store")]
impl Model {
    pub async fn store_id(&self) -> &u8 {
        &self.store_id
    }
    pub async fn manager_staff_id(&self) -> &u8 {
        &self.manager_staff_id
    }
    pub async fn address_id(&self) -> &u16 {
        &self.address_id
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
        &self.last_update
    }
    pub async fn store_store_inventory<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::inventory::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = StoreInventoryFK(self.store_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn store_store_staff<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::staff::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = StoreStaffFK(self.store_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn store_store_customer<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::customer::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = StoreCustomerFK(self.store_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn store_address_address<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::address::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = AddressAddressFK(self.address_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn store_manager_staff_staff<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::staff::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = ManagerStaffStaffFK(self.manager_staff_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "StoreFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub store_id: Option<TypeFilter<u8>>,
    pub manager_staff_id: Option<TypeFilter<u8>>,
    pub address_id: Option<TypeFilter<u16>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StoreInventoryFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StoreInventoryFK> for OrmDataloader {
    type Value = Vec<crate::orm::inventory::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StoreInventoryFK],
    ) -> Result<std::collections::HashMap<StoreInventoryFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::inventory::Column::StoreId.as_column_ref(),
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
                let key = StoreInventoryFK(model.store_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StoreStaffFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StoreStaffFK> for OrmDataloader {
    type Value = Vec<crate::orm::staff::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StoreStaffFK],
    ) -> Result<std::collections::HashMap<StoreStaffFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::staff::Column::StoreId.as_column_ref())
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
        Ok(crate::orm::staff::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = StoreStaffFK(model.store_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StoreCustomerFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StoreCustomerFK> for OrmDataloader {
    type Value = Vec<crate::orm::customer::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StoreCustomerFK],
    ) -> Result<std::collections::HashMap<StoreCustomerFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::customer::Column::StoreId.as_column_ref(),
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
        Ok(crate::orm::customer::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = StoreCustomerFK(model.store_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AddressAddressFK(u16);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<AddressAddressFK> for OrmDataloader {
    type Value = crate::orm::address::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[AddressAddressFK],
    ) -> Result<std::collections::HashMap<AddressAddressFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::address::Column::AddressId.as_column_ref(),
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
        Ok(crate::orm::address::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = AddressAddressFK(model.address_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ManagerStaffStaffFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<ManagerStaffStaffFK> for OrmDataloader {
    type Value = crate::orm::staff::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[ManagerStaffStaffFK],
    ) -> Result<std::collections::HashMap<ManagerStaffStaffFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::staff::Column::StaffId.as_column_ref())
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
        Ok(crate::orm::staff::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = ManagerStaffStaffFK(model.staff_id.clone().try_into().unwrap());
                (key, model)
            })
            .collect())
    }
}
