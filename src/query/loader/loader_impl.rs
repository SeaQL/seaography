//! Mostly copied from SeaORM
use crate::IdenIter;
use sea_orm::{
    dynamic,
    sea_query::{ColumnRef, DynIden, Expr, ExprTrait, IntoColumnRef, TableRef, ValueTuple},
    Condition, ConnectionTrait, DbErr, EntityTrait, Identity, JoinType, ModelTrait, QueryFilter,
    QuerySelect, RelationDef, Select,
};
use std::{collections::HashMap, str::FromStr};

pub trait Container: Default + Clone {
    type Item;
    fn add(&mut self, item: Self::Item);
}

impl<T: Clone> Container for Vec<T> {
    type Item = T;
    fn add(&mut self, item: Self::Item) {
        self.push(item);
    }
}

impl<T: Clone> Container for Option<T> {
    type Item = T;
    fn add(&mut self, item: Self::Item) {
        self.replace(item);
    }
}

pub(super) async fn loader_impl<R, C, T>(
    keys: Vec<ValueTuple>,
    junction_fields: Vec<dynamic::FieldType>,
    stmt: Select<R>,
    rel_def: RelationDef,
    via_def: Option<RelationDef>,
    db: &C,
) -> Result<HashMap<ValueTuple, T>, DbErr>
where
    C: ConnectionTrait,
    R: EntityTrait,
    R::Model: Send + Sync,
    T: Container<Item = R::Model>,
{
    if keys.is_empty() {
        return Ok(Default::default());
    }

    if let Some(via_def) = via_def {
        let condition = prepare_condition(&via_def.to_tbl, &via_def.to_col, &keys)?;

        let stmt = QueryFilter::filter(stmt.join_rev(JoinType::InnerJoin, rel_def), condition);

        // The idea is to do a SelectTwo with join, then extract key via a dynamic model
        // i.e. select (baker + cake_baker) and extract cake_id from result rows
        // SELECT "baker"."id", "baker"."name", "baker"."contact_details", "baker"."bakery_id",
        //     "cakes_bakers"."cake_id" <- extra select
        // FROM "baker" <- target
        // INNER JOIN "cakes_bakers" <- junction
        //     ON "cakes_bakers"."baker_id" = "baker"."id" <- relation
        // WHERE "cakes_bakers"."cake_id" IN (..)

        let data = stmt
            .select_also_dyn_model(
                via_def.to_tbl.sea_orm_table().clone(),
                dynamic::ModelType {
                    fields: junction_fields,
                },
            )
            .all(db)
            .await?;

        let mut hashmap: HashMap<ValueTuple, T> =
            keys.iter()
                .fold(HashMap::new(), |mut acc, key: &ValueTuple| {
                    acc.insert(key.clone(), T::default());
                    acc
                });

        for (item, key) in data {
            let key = dyn_model_to_key(key)?;

            let vec = hashmap.get_mut(&key).ok_or_else(|| {
                DbErr::RecordNotFound(format!("Loader: failed to find model for {key:?}"))
            })?;

            vec.add(item);
        }

        Ok(hashmap)
    } else {
        let condition = prepare_condition(&rel_def.to_tbl, &rel_def.to_col, &keys)?;

        let stmt = QueryFilter::filter(stmt, condition);

        let data = stmt.all(db).await?;

        let mut hashmap: HashMap<ValueTuple, T> = Default::default();

        for item in data {
            let key = extract_key(&rel_def.to_col, &item)?;
            let holder = hashmap.entry(key).or_default();
            holder.add(item);
        }

        Ok(hashmap)
    }
}

pub(crate) fn extract_key<Model>(target_col: &Identity, model: &Model) -> Result<ValueTuple, DbErr>
where
    Model: ModelTrait,
{
    let values = IdenIter::new(target_col)
        .map(|col| {
            let col_name = col.inner();
            let column =
                <<<Model as ModelTrait>::Entity as EntityTrait>::Column as FromStr>::from_str(
                    &col_name,
                )
                .map_err(|_| DbErr::Type(format!("Failed at mapping '{col_name}' to column")))?;
            Ok(model.get(column))
        })
        .collect::<Result<Vec<_>, DbErr>>()?;

    Ok(match values.len() {
        0 => return Err(DbErr::Type("Identity zero?".into())),
        1 => ValueTuple::One(values.into_iter().next().expect("checked")),
        2 => {
            let mut it = values.into_iter();
            ValueTuple::Two(it.next().expect("checked"), it.next().expect("checked"))
        }
        3 => {
            let mut it = values.into_iter();
            ValueTuple::Three(
                it.next().expect("checked"),
                it.next().expect("checked"),
                it.next().expect("checked"),
            )
        }
        _ => ValueTuple::Many(values),
    })
}

pub(crate) fn extract_col_type<Model>(
    left: &Identity,
    right: &Identity,
) -> Result<Vec<dynamic::FieldType>, DbErr>
where
    Model: ModelTrait,
{
    use itertools::Itertools;

    if left.arity() != right.arity() {
        return Err(DbErr::Type(format!(
            "Identity mismatch: left: {} != right: {}",
            left.arity(),
            right.arity()
        )));
    }

    let vec = IdenIter::new(left)
        .zip_eq(IdenIter::new(right))
        .map(|(l, r)| {
            let col_a =
                <<<Model as ModelTrait>::Entity as EntityTrait>::Column as FromStr>::from_str(
                    &l.inner(),
                )
                .map_err(|_| DbErr::Type(format!("Failed at mapping '{l}'")))?;
            Ok(dynamic::FieldType::new(
                r.clone(),
                Model::get_value_type(col_a),
            ))
        })
        .collect::<Result<Vec<_>, DbErr>>()?;

    Ok(vec)
}

#[allow(clippy::unwrap_used)]
fn dyn_model_to_key(dyn_model: dynamic::Model) -> Result<ValueTuple, DbErr> {
    Ok(match dyn_model.fields.len() {
        0 => return Err(DbErr::Type("Identity zero?".into())),
        1 => ValueTuple::One(dyn_model.fields.into_iter().next().unwrap().value),
        2 => {
            let mut iter = dyn_model.fields.into_iter();
            ValueTuple::Two(iter.next().unwrap().value, iter.next().unwrap().value)
        }
        3 => {
            let mut iter = dyn_model.fields.into_iter();
            ValueTuple::Three(
                iter.next().unwrap().value,
                iter.next().unwrap().value,
                iter.next().unwrap().value,
            )
        }
        _ => ValueTuple::Many(dyn_model.fields.into_iter().map(|v| v.value).collect()),
    })
}

fn arity_mismatch(expected: usize, actual: &ValueTuple) -> DbErr {
    DbErr::Type(format!(
        "Loader: arity mismatch: expected {expected}, got {} in {actual:?}",
        actual.arity()
    ))
}

fn prepare_condition(
    table: &TableRef,
    to: &Identity,
    keys: &[ValueTuple],
) -> Result<Condition, DbErr> {
    use itertools::Itertools;

    let arity = to.arity();
    let keys = keys.iter().unique();

    let expr = if arity == 1 {
        let values = keys
            .map(|key| match key {
                ValueTuple::One(v) => Ok(Expr::val(v.to_owned())),
                _ => Err(arity_mismatch(arity, key)),
            })
            .collect::<Result<Vec<_>, DbErr>>()?;

        Expr::col(table_column(table, IdenIter::new(to).next().unwrap())).is_in(values)
    } else {
        let table_columns = create_table_columns(table, to);

        // A vector of tuples of values, e.g. [(v11, v12, ...), (v21, v22, ...), ...]
        let value_tuples = keys
            .map(|key| {
                let key_arity = key.arity();
                if arity != key_arity {
                    return Err(arity_mismatch(arity, key));
                }

                let tuple_exprs = key.clone().into_iter().map(Expr::val);

                Ok(Expr::tuple(tuple_exprs))
            })
            .collect::<Result<Vec<_>, DbErr>>()?;

        // Build `(c1, c2, ...) IN ((v11, v12, ...), (v21, v22, ...), ...)`
        Expr::tuple(table_columns).is_in(value_tuples)
    };

    Ok(expr.into())
}

fn table_column(tbl: &TableRef, col: &DynIden) -> ColumnRef {
    (tbl.sea_orm_table().to_owned(), col.clone()).into_column_ref()
}

/// Create a vector of `Expr::col` from the table and identity, e.g. [Expr::col((table, col1)), Expr::col((table, col2)), ...]
fn create_table_columns(table: &TableRef, cols: &Identity) -> Vec<Expr> {
    IdenIter::new(cols)
        .map(|col| table_column(table, col))
        .map(Expr::col)
        .collect()
}
