use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};
use sea_orm::{EntityTrait, Iterable, JoinType, QuerySelect, RelationTrait};

use crate::{BuilderContext, CascadeBuilder, EntityObjectBuilder, FilterInputBuilder};
use heck::ToUpperCamelCase;

/// The configuration structure for FilterInputBuilder
pub struct CascadeInputConfig {
    /// the filter input type name formatter function
    pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for CascadeInputConfig {
    fn default() -> Self {
        CascadeInputConfig {
            type_name: Box::new(|object_name: &str| -> String {
                format!("{object_name}CascadeInput")
            }),
        }
    }
}

/// This builder is used to produce the filter input object of a SeaORM entity
pub struct CascadeInputBuilder {
    pub context: &'static BuilderContext,
}

impl CascadeInputBuilder {
    /// used to get the filter input object name
    /// object_name is the name of the SeaORM Entity GraphQL object
    pub fn type_name(&self, object_name: &str) -> String {
        self.context.cascade_input.type_name.as_ref()(object_name)
    }

    /// used to produce the filter input object of a SeaORM entity
    pub fn to_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let filter_input_builder = FilterInputBuilder {
            context: self.context,
        };

        let entity_name = entity_object_builder.type_name::<T>();
        let cascade_name = self.type_name(&entity_name);

        let object =
            T::Relation::iter().fold(InputObject::new(&cascade_name), |object, related_table| {
                let related_table_name = related_table.def().to_tbl;
                let fk_name = related_table.def().fk_name;
                let relation_name = if let Some(fk_name) = fk_name {
                    fk_name
                } else {
                    "".to_string()
                };
                if relation_name.is_empty() {
                    return object;
                }
                dbg!(&entity_name);
                match related_table_name {
                    sea_orm::sea_query::TableRef::Table(iden) => {
                        let name = iden.to_string();
                        object.field(InputValue::new(
                            relation_name,
                            TypeRef::named(
                                filter_input_builder.type_name(&name.to_upper_camel_case()),
                            ),
                        ))
                    }
                    sea_orm::sea_query::TableRef::SchemaTable(_, iden) => {
                        let name = iden.to_string();
                        object.field(InputValue::new(
                            relation_name,
                            TypeRef::named(
                                filter_input_builder.type_name(&name.to_upper_camel_case()),
                            ),
                        ))
                    }
                    sea_orm::sea_query::TableRef::DatabaseSchemaTable(_, _, iden) => {
                        let name = iden.to_string().to_upper_camel_case();
                        object.field(InputValue::new(
                            relation_name,
                            TypeRef::named(
                                filter_input_builder.type_name(&name.to_upper_camel_case()),
                            ),
                        ))
                    }
                    _ => object,
                }
            });

        object
    }
    pub fn parse_object<T>(
        &self,
        context: &'static BuilderContext,
        cascades: Option<ObjectAccessor<'_>>,
    ) -> sea_orm::Select<T>
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <T as EntityTrait>::Relation: CascadeBuilder,
    {
        if let Some(cascades) = cascades {
            T::Relation::iter().fold(T::find(), |stmt, relation_definition| {
                let context: &'static BuilderContext = context;
                let fk_name = relation_definition.def().fk_name;
                let relation_name = if let Some(fk_name) = fk_name {
                    fk_name
                } else {
                    "".to_string()
                };

                if let Some(cascade) = cascades.get(&relation_name) {
                    stmt.join(
                        JoinType::InnerJoin,
                        CascadeBuilder::get_join(&relation_definition, &context, Some(cascade)),
                    )
                    .distinct()
                } else {
                    stmt
                }
            })
        } else {
            T::find()
        }
    }
}