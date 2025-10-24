mod impl_traits;
pub(crate) mod loader_impl;

use loader_impl::*;

use sea_orm::{
    sea_query::{Value, ValueTuple},
    EntityTrait, QueryFilter, RelationDef,
};
use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::apply_order;

#[derive(Clone, Debug)]
pub struct KeyComplex<T>
where
    T: EntityTrait,
{
    /// The key tuple to equal with columns
    pub key: ValueTuple,
    /// Meta Information
    pub meta: HashableGroupKey<T>,
}

#[derive(Clone, Debug)]
pub struct HashableGroupKey<T>
where
    T: EntityTrait,
{
    pub stmt: sea_orm::Select<T>,
    pub junction_fields: Vec<sea_orm::dynamic::FieldType>,
    pub rel_def: RelationDef,
    pub via_def: Option<RelationDef>,
    pub filters: sea_orm::Condition,
    pub order_by: Vec<(T::Column, sea_orm::sea_query::Order)>,
}

pub struct OneToManyLoader<T>
where
    T: EntityTrait,
{
    connection: sea_orm::DatabaseConnection,
    entity: PhantomData<T>,
}

impl<T> OneToManyLoader<T>
where
    T: EntityTrait,
    T::Model: Sync,
{
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self {
            connection,
            entity: PhantomData::<T>,
        }
    }
}

impl<T> async_graphql::dataloader::Loader<KeyComplex<T>> for OneToManyLoader<T>
where
    T: EntityTrait,
    T::Model: Sync,
{
    type Value = Vec<T::Model>;
    type Error = std::sync::Arc<sea_orm::DbErr>;

    async fn load(
        &self,
        groups: &[KeyComplex<T>],
    ) -> Result<HashMap<KeyComplex<T>, Self::Value>, Self::Error> {
        let groups = consolidate_groups(groups);

        let mut results: HashMap<KeyComplex<T>, Vec<T::Model>> = HashMap::new();

        for (group, keys) in groups {
            let g = group.clone();
            let mut stmt = g.stmt;
            stmt = stmt.filter(g.filters);
            stmt = apply_order(stmt, g.order_by);
            let models: HashMap<ValueTuple, Vec<T::Model>> = loader_impl(
                keys,
                g.junction_fields,
                stmt,
                g.rel_def,
                g.via_def,
                &self.connection,
            )
            .await?;
            for (key, models) in models {
                results.insert(
                    KeyComplex {
                        key,
                        meta: group.clone(),
                    },
                    models,
                );
            }
        }

        Ok(results)
    }
}

pub struct OneToOneLoader<T>
where
    T: EntityTrait,
{
    connection: sea_orm::DatabaseConnection,
    entity: PhantomData<T>,
}

impl<T> OneToOneLoader<T>
where
    T: EntityTrait,
    T::Model: Sync,
{
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self {
            connection,
            entity: PhantomData::<T>,
        }
    }
}

impl<T> async_graphql::dataloader::Loader<KeyComplex<T>> for OneToOneLoader<T>
where
    T: EntityTrait,
    T::Model: Sync,
{
    type Value = T::Model;
    type Error = std::sync::Arc<sea_orm::DbErr>;

    async fn load(
        &self,
        groups: &[KeyComplex<T>],
    ) -> Result<HashMap<KeyComplex<T>, Self::Value>, Self::Error> {
        let groups = consolidate_groups(groups);

        let mut results: HashMap<KeyComplex<T>, T::Model> = HashMap::new();

        for (group, keys) in groups {
            let g = group.clone();
            let mut stmt = g.stmt;
            stmt = stmt.filter(g.filters);
            stmt = apply_order(stmt, g.order_by);
            let models: HashMap<ValueTuple, Option<T::Model>> = loader_impl(
                keys,
                g.junction_fields,
                stmt,
                g.rel_def,
                g.via_def,
                &self.connection,
            )
            .await?;
            for (key, model) in models {
                if let Some(model) = model {
                    results.insert(
                        KeyComplex {
                            key,
                            meta: group.clone(),
                        },
                        model,
                    );
                }
            }
        }

        Ok(results)
    }
}

fn consolidate_groups<T: EntityTrait>(
    groups: &[KeyComplex<T>],
) -> HashMap<HashableGroupKey<T>, Vec<ValueTuple>> {
    let mut acc: HashMap<HashableGroupKey<T>, Vec<ValueTuple>> = Default::default();

    for cur in groups {
        match acc.get_mut(&cur.meta) {
            Some(items) => {
                items.push(cur.key.clone());
            }
            None => {
                acc.insert(cur.meta.clone(), vec![cur.key.clone()]);
            }
        }
    }

    acc
}
