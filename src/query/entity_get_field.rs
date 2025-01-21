use async_graphql::{
    dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef},
    Error,
};
use heck::ToLowerCamelCase;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Iterable, QueryFilter};

use crate::{BuilderContext, EntityObjectBuilder, GuardAction, TypesMapHelper};

/// The configuration structure for EntityQueryFieldBuilder
pub struct EntityGetFieldConfig {
    /// used to format entity field name
    pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for EntityGetFieldConfig {
    fn default() -> Self {
        EntityGetFieldConfig {
            type_name: Box::new(|object_name: &str| -> String {
                ("get".to_owned() + object_name).to_lower_camel_case()
            }),
        }
    }
}

/// This builder produces a field for the Query object that queries a SeaORM entity
pub struct EntityGetFieldBuilder {
    pub context: &'static BuilderContext,
}

impl EntityGetFieldBuilder {
    /// used to get field name for a SeaORM entity
    pub fn type_name<T>(&self) -> String
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object = EntityObjectBuilder {
            context: self.context,
        };
        let object_name = entity_object.type_name::<T>();
        self.context.entity_get_field.type_name.as_ref()(&object_name)
    }

    /// used to get the Query object field for a SeaORM entity
    pub fn to_field<T>(&self) -> Field
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object = EntityObjectBuilder {
            context: self.context,
        };

        let object_name = entity_object.type_name::<T>();
        let type_ref = TypeRef::named(&object_name);
        let resolver_fn = |object: T::Model| FieldValue::owned_any(object);
        let guard = self.context.guards.entity_guards.get(&object_name);

        let context: &'static BuilderContext = self.context;

        let field = Field::new(self.type_name::<T>(), type_ref, {
            move |ctx| {
                let context: &'static BuilderContext = context;
                FieldFuture::new({
                    async move {
                        let guard_flag = if let Some(guard) = guard {
                            (*guard)(&ctx)
                        } else {
                            GuardAction::Allow
                        };

                        if let GuardAction::Block(reason) = guard_flag {
                            return match reason {
                                Some(reason) => {
                                    Err::<Option<_>, async_graphql::Error>(Error::new(reason))
                                }
                                None => Err::<Option<_>, async_graphql::Error>(Error::new(
                                    "Entity guard triggered.",
                                )),
                            };
                        }

                        let stmt = T::find();
                        let stmt = T::Column::iter().fold(stmt, |stmt, column| {
                            let entity_object_builder = EntityObjectBuilder { context };
                            let column_name = entity_object_builder.column_name::<T>(&column);
                            let types_map_helper = TypesMapHelper { context };
                            match ctx.args.get(&column_name) {
                                Some(val) => stmt.filter(
                                    column.eq(types_map_helper
                                        .async_graphql_value_to_sea_orm_value::<T>(&column, &val)
                                        .unwrap()),
                                ),
                                _ => stmt,
                            }
                        });

                        let db = ctx.data::<DatabaseConnection>()?;

                        let object = stmt.one(db).await?;

                        match object {
                            Some(object) => Ok(Some(resolver_fn(object))),
                            _ => Ok(Some(FieldValue::NULL)),
                        }
                    }
                })
            }
        });

        T::Column::iter().fold(field, |field, column| {
            let column_name = entity_object.column_name::<T>(&column);
            let types_map_helper = TypesMapHelper { context };
            field.argument(InputValue::new(
                column_name,
                types_map_helper
                    .sea_orm_column_type_to_graphql_type(column.def().get_column_type(), false)
                    .unwrap(),
            ))
        })
    }
}