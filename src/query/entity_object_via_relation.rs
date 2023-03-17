use async_graphql::dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef};
use heck::{ToSnakeCase, ToUpperCamelCase};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Iden, ModelTrait, QueryFilter, Related,
};

use crate::{apply_order, apply_pagination, get_filter_conditions, BuilderContext};

#[derive(Clone, Debug)]
pub struct EntityObjectViaRelationBuilder {
    pub context: &'static BuilderContext,
}

impl EntityObjectViaRelationBuilder {
    // FIXME: read names from context
    pub fn get_relation<T, R>(&self, name: &str) -> Field
    where
        T: Related<R>,
        T: EntityTrait,
        R: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <R as sea_orm::EntityTrait>::Model: Sync,
        <<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
        <<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
    {
        let context: &'static BuilderContext = self.context;
        let to_relation_definition = <T as Related<R>>::to();
        let via_relation_definition = <T as Related<R>>::via().expect(
            "We expect this function to be used with Related that has `via` method implemented!",
        );

        let type_name: String = match to_relation_definition.to_tbl {
            sea_orm::sea_query::TableRef::Table(table) => table.to_string(),
            sea_orm::sea_query::TableRef::TableAlias(table, _alias) => table.to_string(),
            sea_orm::sea_query::TableRef::SchemaTable(_schema, table) => table.to_string(),
            sea_orm::sea_query::TableRef::DatabaseSchemaTable(_database, _schema, table) => {
                table.to_string()
            }
            sea_orm::sea_query::TableRef::SchemaTableAlias(_schema, table, _alias) => {
                table.to_string()
            }
            sea_orm::sea_query::TableRef::DatabaseSchemaTableAlias(
                _database,
                _schema,
                table,
                _alias,
            ) => table.to_string(),
            // FIXME: what if empty ?
            sea_orm::sea_query::TableRef::SubQuery(_stmt, alias) => alias.to_string(),
            sea_orm::sea_query::TableRef::ValuesList(_values, alias) => alias.to_string(),
            sea_orm::sea_query::TableRef::FunctionCall(_, alias) => alias.to_string(),
        }
        .to_upper_camel_case();

        let from_col = <T::Column as std::str::FromStr>::from_str(
            via_relation_definition
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();

        let to_col = <R::Column as std::str::FromStr>::from_str(
            to_relation_definition
                .to_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();

        let field = match via_relation_definition.is_owner {
            false => {
                Field::new(name, TypeRef::named(type_name.to_string()), move |ctx| {
                    // FIXME: optimize by adding dataloader
                    FieldFuture::new(async move {
                        let parent: &T::Model = ctx
                            .parent_value
                            .try_downcast_ref::<T::Model>()
                            .expect("Parent should exist");

                        let stmt = if <T as Related<R>>::via().is_some() {
                            <T as Related<R>>::find_related()
                        } else {
                            R::find()
                        };

                        let filter = Condition::all().add(to_col.eq(parent.get(from_col)));

                        let stmt = stmt.filter(filter);

                        let db = ctx.data::<DatabaseConnection>()?;

                        let data = stmt.one(db).await?;

                        if let Some(data) = data {
                            Ok(Some(FieldValue::owned_any(data)))
                        } else {
                            Ok(None)
                        }
                    })
                })
            }
            true => Field::new(
                name,
                TypeRef::named_nn(format!("{}Connection", type_name)),
                move |ctx| {
                    let context: &'static BuilderContext = context;
                    FieldFuture::new(async move {
                        // FIXME: optimize union queries
                        // NOTE: each has unique query in order to apply pagination...
                        let parent: &T::Model = ctx
                            .parent_value
                            .try_downcast_ref::<T::Model>()
                            .expect("Parent should exist");

                        let stmt = if <T as Related<R>>::via().is_some() {
                            <T as Related<R>>::find_related()
                        } else {
                            R::find()
                        };

                        let condition = Condition::all().add(to_col.eq(parent.get(from_col)));

                        let filters = ctx.args.get("filters");
                        let order_by = ctx.args.get("orderBy");
                        let pagination = ctx.args.get("pagination");

                        let base_condition = get_filter_conditions::<R>(filters);

                        let stmt = stmt.filter(condition.add(base_condition));
                        let stmt = apply_order(context, stmt, order_by);

                        let db = ctx.data::<DatabaseConnection>()?;

                        let connection =
                            apply_pagination::<R>(context, db, stmt, pagination).await?;

                        Ok(Some(FieldValue::owned_any(connection)))
                    })
                },
            ),
        };

        let field = match via_relation_definition.is_owner {
            false => field,
            true => field
                .argument(InputValue::new(
                    "filters",
                    TypeRef::named(format!("{}FilterInput", type_name)),
                ))
                .argument(InputValue::new(
                    "orderBy",
                    TypeRef::named(format!("{}OrderInput", type_name)),
                ))
                .argument(InputValue::new(
                    "pagination",
                    TypeRef::named("PaginationInput"),
                )),
        };

        field
    }
}
