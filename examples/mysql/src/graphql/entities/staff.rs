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
        if let Some(staff_id) = current_filter.staff_id {
            if let Some(eq_value) = staff_id.eq {
                condition = condition.add(Column::StaffId.eq(eq_value))
            }
            if let Some(ne_value) = staff_id.ne {
                condition = condition.add(Column::StaffId.ne(ne_value))
            }
            if let Some(gt_value) = staff_id.gt {
                condition = condition.add(Column::StaffId.gt(gt_value))
            }
            if let Some(gte_value) = staff_id.gte {
                condition = condition.add(Column::StaffId.gte(gte_value))
            }
            if let Some(lt_value) = staff_id.lt {
                condition = condition.add(Column::StaffId.lt(lt_value))
            }
            if let Some(lte_value) = staff_id.lte {
                condition = condition.add(Column::StaffId.lte(lte_value))
            }
            if let Some(is_in_value) = staff_id.is_in {
                condition = condition.add(Column::StaffId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = staff_id.is_not_in {
                condition = condition.add(Column::StaffId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = staff_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::StaffId.is_null())
                }
            }
        }
        if let Some(first_name) = current_filter.first_name {
            if let Some(eq_value) = first_name.eq {
                condition = condition.add(Column::FirstName.eq(eq_value))
            }
            if let Some(ne_value) = first_name.ne {
                condition = condition.add(Column::FirstName.ne(ne_value))
            }
            if let Some(gt_value) = first_name.gt {
                condition = condition.add(Column::FirstName.gt(gt_value))
            }
            if let Some(gte_value) = first_name.gte {
                condition = condition.add(Column::FirstName.gte(gte_value))
            }
            if let Some(lt_value) = first_name.lt {
                condition = condition.add(Column::FirstName.lt(lt_value))
            }
            if let Some(lte_value) = first_name.lte {
                condition = condition.add(Column::FirstName.lte(lte_value))
            }
            if let Some(is_in_value) = first_name.is_in {
                condition = condition.add(Column::FirstName.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = first_name.is_not_in {
                condition = condition.add(Column::FirstName.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = first_name.is_null {
                if is_null_value {
                    condition = condition.add(Column::FirstName.is_null())
                }
            }
        }
        if let Some(last_name) = current_filter.last_name {
            if let Some(eq_value) = last_name.eq {
                condition = condition.add(Column::LastName.eq(eq_value))
            }
            if let Some(ne_value) = last_name.ne {
                condition = condition.add(Column::LastName.ne(ne_value))
            }
            if let Some(gt_value) = last_name.gt {
                condition = condition.add(Column::LastName.gt(gt_value))
            }
            if let Some(gte_value) = last_name.gte {
                condition = condition.add(Column::LastName.gte(gte_value))
            }
            if let Some(lt_value) = last_name.lt {
                condition = condition.add(Column::LastName.lt(lt_value))
            }
            if let Some(lte_value) = last_name.lte {
                condition = condition.add(Column::LastName.lte(lte_value))
            }
            if let Some(is_in_value) = last_name.is_in {
                condition = condition.add(Column::LastName.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = last_name.is_not_in {
                condition = condition.add(Column::LastName.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = last_name.is_null {
                if is_null_value {
                    condition = condition.add(Column::LastName.is_null())
                }
            }
        }
        if let Some(address_id) = current_filter.address_id {
            if let Some(eq_value) = address_id.eq {
                condition = condition.add(Column::AddressId.eq(eq_value))
            }
            if let Some(ne_value) = address_id.ne {
                condition = condition.add(Column::AddressId.ne(ne_value))
            }
            if let Some(gt_value) = address_id.gt {
                condition = condition.add(Column::AddressId.gt(gt_value))
            }
            if let Some(gte_value) = address_id.gte {
                condition = condition.add(Column::AddressId.gte(gte_value))
            }
            if let Some(lt_value) = address_id.lt {
                condition = condition.add(Column::AddressId.lt(lt_value))
            }
            if let Some(lte_value) = address_id.lte {
                condition = condition.add(Column::AddressId.lte(lte_value))
            }
            if let Some(is_in_value) = address_id.is_in {
                condition = condition.add(Column::AddressId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = address_id.is_not_in {
                condition = condition.add(Column::AddressId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = address_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::AddressId.is_null())
                }
            }
        }
        if let Some(email) = current_filter.email {
            if let Some(eq_value) = email.eq {
                condition = condition.add(Column::Email.eq(eq_value))
            }
            if let Some(ne_value) = email.ne {
                condition = condition.add(Column::Email.ne(ne_value))
            }
            if let Some(gt_value) = email.gt {
                condition = condition.add(Column::Email.gt(gt_value))
            }
            if let Some(gte_value) = email.gte {
                condition = condition.add(Column::Email.gte(gte_value))
            }
            if let Some(lt_value) = email.lt {
                condition = condition.add(Column::Email.lt(lt_value))
            }
            if let Some(lte_value) = email.lte {
                condition = condition.add(Column::Email.lte(lte_value))
            }
            if let Some(is_in_value) = email.is_in {
                condition = condition.add(Column::Email.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = email.is_not_in {
                condition = condition.add(Column::Email.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = email.is_null {
                if is_null_value {
                    condition = condition.add(Column::Email.is_null())
                }
            }
        }
        if let Some(store_id) = current_filter.store_id {
            if let Some(eq_value) = store_id.eq {
                condition = condition.add(Column::StoreId.eq(eq_value))
            }
            if let Some(ne_value) = store_id.ne {
                condition = condition.add(Column::StoreId.ne(ne_value))
            }
            if let Some(gt_value) = store_id.gt {
                condition = condition.add(Column::StoreId.gt(gt_value))
            }
            if let Some(gte_value) = store_id.gte {
                condition = condition.add(Column::StoreId.gte(gte_value))
            }
            if let Some(lt_value) = store_id.lt {
                condition = condition.add(Column::StoreId.lt(lt_value))
            }
            if let Some(lte_value) = store_id.lte {
                condition = condition.add(Column::StoreId.lte(lte_value))
            }
            if let Some(is_in_value) = store_id.is_in {
                condition = condition.add(Column::StoreId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = store_id.is_not_in {
                condition = condition.add(Column::StoreId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = store_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::StoreId.is_null())
                }
            }
        }
        if let Some(active) = current_filter.active {
            if let Some(eq_value) = active.eq {
                condition = condition.add(Column::Active.eq(eq_value))
            }
            if let Some(ne_value) = active.ne {
                condition = condition.add(Column::Active.ne(ne_value))
            }
            if let Some(gt_value) = active.gt {
                condition = condition.add(Column::Active.gt(gt_value))
            }
            if let Some(gte_value) = active.gte {
                condition = condition.add(Column::Active.gte(gte_value))
            }
            if let Some(lt_value) = active.lt {
                condition = condition.add(Column::Active.lt(lt_value))
            }
            if let Some(lte_value) = active.lte {
                condition = condition.add(Column::Active.lte(lte_value))
            }
            if let Some(is_in_value) = active.is_in {
                condition = condition.add(Column::Active.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = active.is_not_in {
                condition = condition.add(Column::Active.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = active.is_null {
                if is_null_value {
                    condition = condition.add(Column::Active.is_null())
                }
            }
        }
        if let Some(username) = current_filter.username {
            if let Some(eq_value) = username.eq {
                condition = condition.add(Column::Username.eq(eq_value))
            }
            if let Some(ne_value) = username.ne {
                condition = condition.add(Column::Username.ne(ne_value))
            }
            if let Some(gt_value) = username.gt {
                condition = condition.add(Column::Username.gt(gt_value))
            }
            if let Some(gte_value) = username.gte {
                condition = condition.add(Column::Username.gte(gte_value))
            }
            if let Some(lt_value) = username.lt {
                condition = condition.add(Column::Username.lt(lt_value))
            }
            if let Some(lte_value) = username.lte {
                condition = condition.add(Column::Username.lte(lte_value))
            }
            if let Some(is_in_value) = username.is_in {
                condition = condition.add(Column::Username.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = username.is_not_in {
                condition = condition.add(Column::Username.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = username.is_null {
                if is_null_value {
                    condition = condition.add(Column::Username.is_null())
                }
            }
        }
        if let Some(password) = current_filter.password {
            if let Some(eq_value) = password.eq {
                condition = condition.add(Column::Password.eq(eq_value))
            }
            if let Some(ne_value) = password.ne {
                condition = condition.add(Column::Password.ne(ne_value))
            }
            if let Some(gt_value) = password.gt {
                condition = condition.add(Column::Password.gt(gt_value))
            }
            if let Some(gte_value) = password.gte {
                condition = condition.add(Column::Password.gte(gte_value))
            }
            if let Some(lt_value) = password.lt {
                condition = condition.add(Column::Password.lt(lt_value))
            }
            if let Some(lte_value) = password.lte {
                condition = condition.add(Column::Password.lte(lte_value))
            }
            if let Some(is_in_value) = password.is_in {
                condition = condition.add(Column::Password.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = password.is_not_in {
                condition = condition.add(Column::Password.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = password.is_null {
                if is_null_value {
                    condition = condition.add(Column::Password.is_null())
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
pub use crate::orm::staff::*;
#[async_graphql::Object(name = "Staff")]
impl Model {
    pub async fn staff_id(&self) -> &u8 {
        &self.staff_id
    }
    pub async fn first_name(&self) -> &String {
        &self.first_name
    }
    pub async fn last_name(&self) -> &String {
        &self.last_name
    }
    pub async fn address_id(&self) -> &u16 {
        &self.address_id
    }
    pub async fn picture(&self) -> &Option<Vec<u8>> {
        &self.picture
    }
    pub async fn email(&self) -> &Option<String> {
        &self.email
    }
    pub async fn store_id(&self) -> &u8 {
        &self.store_id
    }
    pub async fn active(&self) -> &i8 {
        &self.active
    }
    pub async fn username(&self) -> &String {
        &self.username
    }
    pub async fn password(&self) -> &Option<String> {
        &self.password
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
        &self.last_update
    }
    pub async fn staff_staff_payment<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::payment::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = StaffPaymentFK(self.staff_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn staff_staff_store<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::store::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = StaffStoreFK(self.staff_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn staff_staff_rental<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::rental::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = StaffRentalFK(self.staff_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn staff_address_address<'a>(
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
    pub async fn staff_store_store<'a>(
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
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "StaffFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub staff_id: Option<TypeFilter<u8>>,
    pub first_name: Option<TypeFilter<String>>,
    pub last_name: Option<TypeFilter<String>>,
    pub address_id: Option<TypeFilter<u16>>,
    pub email: Option<TypeFilter<String>>,
    pub store_id: Option<TypeFilter<u8>>,
    pub active: Option<TypeFilter<i8>>,
    pub username: Option<TypeFilter<String>>,
    pub password: Option<TypeFilter<String>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StaffPaymentFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StaffPaymentFK> for OrmDataloader {
    type Value = Vec<crate::orm::payment::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StaffPaymentFK],
    ) -> Result<std::collections::HashMap<StaffPaymentFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::payment::Column::StaffId.as_column_ref())
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
        Ok(crate::orm::payment::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = StaffPaymentFK(model.staff_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StaffStoreFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StaffStoreFK> for OrmDataloader {
    type Value = Vec<crate::orm::store::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StaffStoreFK],
    ) -> Result<std::collections::HashMap<StaffStoreFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::store::Column::ManagerStaffId.as_column_ref(),
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
        Ok(crate::orm::store::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = StaffStoreFK(model.manager_staff_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StaffRentalFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StaffRentalFK> for OrmDataloader {
    type Value = Vec<crate::orm::rental::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StaffRentalFK],
    ) -> Result<std::collections::HashMap<StaffRentalFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::rental::Column::StaffId.as_column_ref())
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
                let key = StaffRentalFK(model.staff_id.clone().try_into().unwrap());
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
