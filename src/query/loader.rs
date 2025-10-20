mod impl_traits;
use impl_traits::*;

use sea_orm::{
    sea_query::{Value, ValueTuple},
    Condition, EntityTrait, ExprTrait, ModelTrait, QueryFilter,
};
use std::{collections::HashMap, hash::Hash, marker::PhantomData, sync::Arc};

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
    /// Foundation SQL statement
    pub stmt: sea_orm::Select<T>,
    /// Columns tuple
    pub columns: Vec<T::Column>,
    /// Extra `WHERE` condition
    pub filters: Option<sea_orm::Condition>,
    /// Ordering
    pub order_by: Vec<(T::Column, sea_orm::sea_query::Order)>,
}

#[derive(Clone, Debug)]
pub struct HashableColumn<T>(T::Column)
where
    T: EntityTrait;

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
        keys: &[KeyComplex<T>],
    ) -> Result<HashMap<KeyComplex<T>, Self::Value>, Self::Error> {
        let items: HashMap<HashableGroupKey<T>, Vec<ValueTuple>> = keys
            .iter()
            .cloned()
            .map(|item: KeyComplex<T>| {
                (
                    HashableGroupKey {
                        stmt: item.meta.stmt,
                        columns: item.meta.columns,
                        filters: item.meta.filters,
                        order_by: item.meta.order_by,
                    },
                    item.key,
                )
            })
            .fold(
                HashMap::<HashableGroupKey<T>, Vec<ValueTuple>>::new(),
                |mut acc: HashMap<HashableGroupKey<T>, Vec<ValueTuple>>,
                 cur: (HashableGroupKey<T>, ValueTuple)| {
                    match acc.get_mut(&cur.0) {
                        Some(items) => {
                            items.push(cur.1);
                        }
                        None => {
                            acc.insert(cur.0, vec![cur.1]);
                        }
                    }

                    acc
                },
            );

        let promises: HashMap<HashableGroupKey<T>, _> = items
            .into_iter()
            .map(|(key, values): (HashableGroupKey<T>, Vec<ValueTuple>)| {
                let cloned_key = key.clone();

                let stmt = key.stmt;

                let condition = match key.filters {
                    Some(condition) => Condition::all().add(condition),
                    None => Condition::all(),
                };
                let condition = apply_condition(condition, &key.columns, values);
                let stmt = stmt.filter(condition);

                let stmt = apply_order(stmt, key.order_by);

                (cloned_key, stmt.all(&self.connection))
            })
            .collect();

        let mut results: HashMap<KeyComplex<T>, Vec<T::Model>> = HashMap::new();

        for (key, promise) in promises.into_iter() {
            let key = key as HashableGroupKey<T>;
            let result: Vec<T::Model> = promise.await.map_err(Arc::new)?;
            for item in result.into_iter() {
                let key = &KeyComplex::<T> {
                    key: collect_key(key.columns.iter().map(|col: &T::Column| item.get(*col))),
                    meta: key.clone(),
                };
                match results.get_mut(key) {
                    Some(results) => {
                        results.push(item);
                    }
                    None => {
                        results.insert(key.clone(), vec![item]);
                    }
                };
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
        keys: &[KeyComplex<T>],
    ) -> Result<HashMap<KeyComplex<T>, Self::Value>, Self::Error> {
        let items: HashMap<HashableGroupKey<T>, Vec<ValueTuple>> = keys
            .iter()
            .cloned()
            .map(|item: KeyComplex<T>| {
                (
                    HashableGroupKey {
                        stmt: item.meta.stmt,
                        columns: item.meta.columns,
                        filters: item.meta.filters,
                        order_by: item.meta.order_by,
                    },
                    item.key,
                )
            })
            .fold(
                HashMap::<HashableGroupKey<T>, Vec<ValueTuple>>::new(),
                |mut acc: HashMap<HashableGroupKey<T>, Vec<ValueTuple>>,
                 cur: (HashableGroupKey<T>, ValueTuple)| {
                    match acc.get_mut(&cur.0) {
                        Some(items) => {
                            items.push(cur.1);
                        }
                        None => {
                            acc.insert(cur.0, vec![cur.1]);
                        }
                    }

                    acc
                },
            );

        let promises: HashMap<HashableGroupKey<T>, _> = items
            .into_iter()
            .map(|(key, values): (HashableGroupKey<T>, Vec<ValueTuple>)| {
                let cloned_key = key.clone();

                let stmt = key.stmt;

                let condition = match key.filters {
                    Some(condition) => Condition::all().add(condition),
                    None => Condition::all(),
                };
                let condition = apply_condition(condition, &key.columns, values);
                let stmt = stmt.filter(condition);

                let stmt = apply_order(stmt, key.order_by);

                (cloned_key, stmt.all(&self.connection))
            })
            .collect();

        let mut results: HashMap<KeyComplex<T>, T::Model> = HashMap::new();

        for (key, promise) in promises.into_iter() {
            let key = key as HashableGroupKey<T>;
            let result: Vec<T::Model> = promise.await.map_err(Arc::new)?;
            for item in result.into_iter() {
                let key = &KeyComplex::<T> {
                    key: collect_key(key.columns.iter().map(|col: &T::Column| item.get(*col))),
                    meta: key.clone(),
                };
                results.insert(key.clone(), item);
            }
        }

        Ok(results)
    }
}

fn apply_condition<T: sea_orm::ColumnTrait>(
    condition: Condition,
    cols: &[T],
    values: Vec<ValueTuple>,
) -> Condition {
    if cols.len() == 1 {
        condition.add(
            sea_orm::sea_query::Expr::col(cols[0]).is_in(values.into_iter().map(
                |tuple| match tuple {
                    ValueTuple::One(v) => v,
                    ValueTuple::Two(v, _) => v,
                    ValueTuple::Three(v, _, _) => v,
                    _ => panic!("Column & Value arity mismatch: expected 1"),
                },
            )),
        )
    } else {
        let tuple = sea_orm::sea_query::Expr::tuple(
            cols.iter()
                .map(|column| sea_orm::sea_query::Expr::col(*column)),
        );
        condition.add(tuple.in_tuples(values))
    }
}
