use sea_orm::{sea_query::ValueTuple, Condition, ModelTrait, QueryFilter};
use std::{collections::HashMap, hash::Hash, marker::PhantomData, sync::Arc};

use crate::apply_order;

#[derive(Clone)]
pub struct KeyComplex<T>
where
    T: sea_orm::EntityTrait,
{
    /// The key tuple to equal with columns
    pub key: Vec<sea_orm::Value>,
    /// Meta Information
    pub meta: HashAbleGroupKey<T>,
}

impl<T> PartialEq for KeyComplex<T>
where
    T: sea_orm::EntityTrait,
{
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key) && self.meta.eq(&other.meta)
    }
}

impl<T> Eq for KeyComplex<T> where T: sea_orm::EntityTrait {}

impl<T> Hash for KeyComplex<T>
where
    T: sea_orm::EntityTrait,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.meta.hash(state);
    }
}

#[derive(Clone)]
pub struct HashAbleGroupKey<T>
where
    T: sea_orm::EntityTrait,
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

impl<T> PartialEq for HashAbleGroupKey<T>
where
    T: sea_orm::EntityTrait,
{
    fn eq(&self, other: &Self) -> bool {
        self.filters.eq(&other.filters)
            && format!("{:?}", self.columns).eq(&format!("{:?}", other.columns))
            && format!("{:?}", self.order_by).eq(&format!("{:?}", other.order_by))
    }
}

impl<T> Eq for HashAbleGroupKey<T> where T: sea_orm::EntityTrait {}

impl<T> Hash for HashAbleGroupKey<T>
where
    T: sea_orm::EntityTrait,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{:?}", self.filters).hash(state);
        format!("{:?}", self.columns).hash(state);
        format!("{:?}", self.order_by).hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct HashAbleColumn<T>(T::Column)
where
    T: sea_orm::EntityTrait;

impl<T> PartialEq for HashAbleColumn<T>
where
    T: sea_orm::EntityTrait,
{
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self.0).eq(&format!("{:?}", other.0))
    }
}

impl<T> Eq for HashAbleColumn<T> where T: sea_orm::EntityTrait {}

impl<T> Hash for HashAbleColumn<T>
where
    T: sea_orm::EntityTrait,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{:?}", self.0).hash(state);
    }
}

pub struct OneToManyLoader<T>
where
    T: sea_orm::EntityTrait,
{
    connection: sea_orm::DatabaseConnection,
    entity: PhantomData<T>,
}

impl<T> OneToManyLoader<T>
where
    T: sea_orm::EntityTrait,
    T::Model: Sync,
{
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self {
            connection,
            entity: PhantomData::<T>,
        }
    }
}

#[async_trait::async_trait]
impl<T> async_graphql::dataloader::Loader<KeyComplex<T>> for OneToManyLoader<T>
where
    T: sea_orm::EntityTrait,
    T::Model: Sync,
{
    type Value = Vec<T::Model>;
    type Error = std::sync::Arc<sea_orm::DbErr>;

    async fn load(
        &self,
        keys: &[KeyComplex<T>],
    ) -> Result<HashMap<KeyComplex<T>, Self::Value>, Self::Error> {
        let items: HashMap<HashAbleGroupKey<T>, Vec<Vec<sea_orm::Value>>> = keys
            .iter()
            .cloned()
            .map(|item: KeyComplex<T>| {
                (
                    HashAbleGroupKey {
                        stmt: item.meta.stmt,
                        columns: item.meta.columns,
                        filters: item.meta.filters,
                        order_by: item.meta.order_by,
                    },
                    item.key,
                )
            })
            .fold(
                HashMap::<HashAbleGroupKey<T>, Vec<Vec<sea_orm::Value>>>::new(),
                |mut acc: HashMap<HashAbleGroupKey<T>, Vec<Vec<sea_orm::Value>>>,
                 cur: (HashAbleGroupKey<T>, Vec<sea_orm::Value>)| {
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

        let promises: HashMap<HashAbleGroupKey<T>, _> = items
            .into_iter()
            .map(
                |(key, values): (HashAbleGroupKey<T>, Vec<Vec<sea_orm::Value>>)| {
                    let cloned_key = key.clone();

                    let stmt = key.stmt;

                    let condition = match key.filters {
                        Some(condition) => Condition::all().add(condition),
                        None => Condition::all(),
                    };
                    let tuple =
                        sea_orm::sea_query::Expr::tuple(key.columns.iter().map(
                            |column: &T::Column| sea_orm::sea_query::Expr::col(*column).into(),
                        ));
                    let condition =
                        condition.add(tuple.in_tuples(values.into_iter().map(ValueTuple::Many)));
                    let stmt = stmt.filter(condition);

                    let stmt = apply_order(stmt, key.order_by);

                    (cloned_key, stmt.all(&self.connection))
                },
            )
            .collect();

        let mut results: HashMap<KeyComplex<T>, Vec<T::Model>> = HashMap::new();

        for (key, promise) in promises.into_iter() {
            let key = key as HashAbleGroupKey<T>;
            let result: Vec<T::Model> = promise.await.map_err(Arc::new)?;
            for item in result.into_iter() {
                let key = &KeyComplex::<T> {
                    key: key
                        .columns
                        .iter()
                        .map(|col: &T::Column| item.get(*col))
                        .collect(),
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
    T: sea_orm::EntityTrait,
{
    connection: sea_orm::DatabaseConnection,
    entity: PhantomData<T>,
}

impl<T> OneToOneLoader<T>
where
    T: sea_orm::EntityTrait,
    T::Model: Sync,
{
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self {
            connection,
            entity: PhantomData::<T>,
        }
    }
}

#[async_trait::async_trait]
impl<T> async_graphql::dataloader::Loader<KeyComplex<T>> for OneToOneLoader<T>
where
    T: sea_orm::EntityTrait,
    T::Model: Sync,
{
    type Value = T::Model;
    type Error = std::sync::Arc<sea_orm::DbErr>;

    async fn load(
        &self,
        keys: &[KeyComplex<T>],
    ) -> Result<HashMap<KeyComplex<T>, Self::Value>, Self::Error> {
        let items: HashMap<HashAbleGroupKey<T>, Vec<Vec<sea_orm::Value>>> = keys
            .iter()
            .cloned()
            .map(|item: KeyComplex<T>| {
                (
                    HashAbleGroupKey {
                        stmt: item.meta.stmt,
                        columns: item.meta.columns,
                        filters: item.meta.filters,
                        order_by: item.meta.order_by,
                    },
                    item.key,
                )
            })
            .fold(
                HashMap::<HashAbleGroupKey<T>, Vec<Vec<sea_orm::Value>>>::new(),
                |mut acc: HashMap<HashAbleGroupKey<T>, Vec<Vec<sea_orm::Value>>>,
                 cur: (HashAbleGroupKey<T>, Vec<sea_orm::Value>)| {
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

        let promises: HashMap<HashAbleGroupKey<T>, _> = items
            .into_iter()
            .map(
                |(key, values): (HashAbleGroupKey<T>, Vec<Vec<sea_orm::Value>>)| {
                    let cloned_key = key.clone();

                    let stmt = key.stmt;

                    let condition = match key.filters {
                        Some(condition) => Condition::all().add(condition),
                        None => Condition::all(),
                    };
                    let tuple =
                        sea_orm::sea_query::Expr::tuple(key.columns.iter().map(
                            |column: &T::Column| sea_orm::sea_query::Expr::col(*column).into(),
                        ));
                    let condition =
                        condition.add(tuple.in_tuples(values.into_iter().map(ValueTuple::Many)));
                    let stmt = stmt.filter(condition);

                    let stmt = apply_order(stmt, key.order_by);

                    (cloned_key, stmt.all(&self.connection))
                },
            )
            .collect();

        let mut results: HashMap<KeyComplex<T>, T::Model> = HashMap::new();

        for (key, promise) in promises.into_iter() {
            let key = key as HashAbleGroupKey<T>;
            let result: Vec<T::Model> = promise.await.map_err(Arc::new)?;
            for item in result.into_iter() {
                let key = &KeyComplex::<T> {
                    key: key
                        .columns
                        .iter()
                        .map(|col: &T::Column| item.get(*col))
                        .collect(),
                    meta: key.clone(),
                };
                results.insert(key.clone(), item);
            }
        }

        Ok(results)
    }
}
