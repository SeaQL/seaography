use async_graphql::dynamic::{Enum, Field, InputObject, Object, Schema, SchemaBuilder};
use sea_orm::{ActiveEnum, EntityTrait, IntoActiveModel, ActiveModelTrait};

use crate::{
    ActiveEnumBuilder, ActiveEnumFilterInputBuilder, BuilderContext, ConnectionObjectBuilder,
    CursorInputBuilder, EdgeObjectBuilder, EntityObjectBuilder, EntityQueryFieldBuilder,
    FilterInputBuilder, OffsetInputBuilder, OrderByEnumBuilder, OrderInputBuilder,
    PageInfoObjectBuilder, PageInputBuilder, PaginationInfoObjectBuilder, PaginationInputBuilder, EntityCreateOneMutationBuilder, EntityInputBuilder,
};

/// The Builder is used to create the Schema for GraphQL
///
/// You can populate it with the entities, enumerations of your choice
pub struct Builder {
    /// holds all output object types
    pub outputs: Vec<Object>,

    /// holds all input object types
    pub inputs: Vec<InputObject>,

    /// holds all enumeration types
    pub enumerations: Vec<Enum>,

    /// holds all entities queries
    pub queries: Vec<Field>,

    /// holds all entities mutations
    pub mutations: Vec<Field>,

    /// configuration for builder
    pub context: &'static BuilderContext,
}

impl Builder {
    /// Used to create a new Builder from the given configuration context
    pub fn new(context: &'static BuilderContext) -> Self {
        Self {
            outputs: Vec::new(),
            inputs: Vec::new(),
            enumerations: Vec::new(),
            queries: Vec::new(),
            mutations: Vec::new(),
            context,
        }
    }

    /// used to register a new entity to the Builder context
    pub fn register_entity<T>(&mut self, relations: Vec<Field>)
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let entity_object = relations
            .into_iter()
            .fold(entity_object_builder.to_object::<T>(), |entity_object, field| {
                entity_object.field(field)
            });

        let edge_object_builder = EdgeObjectBuilder {
            context: self.context,
        };
        let edge = edge_object_builder.to_object::<T>();

        let connection_object_builder = ConnectionObjectBuilder {
            context: self.context,
        };
        let connection = connection_object_builder.to_object::<T>();

        self.outputs.extend(vec![entity_object, edge, connection]);


        let filter_input_builder = FilterInputBuilder {
            context: self.context,
        };
        let filter = filter_input_builder.to_object::<T>();

        let order_input_builder = OrderInputBuilder {
            context: self.context,
        };
        let order = order_input_builder.to_object::<T>();
        self.inputs.extend(vec![filter, order]);

        let entity_query_field_builder = EntityQueryFieldBuilder {
            context: self.context,
        };
        let query = entity_query_field_builder.to_field::<T>();
        self.queries.push(query);
    }

    pub fn register_entity_mutations<T, A>(&mut self)
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <T as EntityTrait>::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let basic_entity_object = entity_object_builder.basic_to_object::<T>();
        self.outputs.push(basic_entity_object);

        let entity_input_builder = EntityInputBuilder { context: self.context };

        if self.context.entity_input.unified {
            let entity_input_object = entity_input_builder.insert_input_object::<T>();
            self.inputs.push(entity_input_object);
        } else {
            let entity_insert_input_object = entity_input_builder.insert_input_object::<T>();
            let entity_update_input_object = entity_input_builder.update_input_object::<T>();
            self.inputs.extend(vec![entity_insert_input_object, entity_update_input_object]);
        }

        let entity_create_one_mutation_builder = EntityCreateOneMutationBuilder {
            context: self.context,
        };
        let create_mutation = entity_create_one_mutation_builder.to_field::<T, A>();
        self.mutations.push(create_mutation);
    }


    /// used to register a new enumeration to the builder context
    pub fn register_enumeration<A>(&mut self)
    where
        A: ActiveEnum,
    {
        let active_enum_builder = ActiveEnumBuilder {
            context: self.context,
        };
        let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder {
            context: self.context,
        };

        let enumeration = active_enum_builder.enumeration::<A>();
        self.enumerations.push(enumeration);

        let filter = active_enum_filter_input_builder.input_object::<A>();
        self.inputs.push(filter);
    }

    /// used to consume the builder context and generate a ready to be completed GraphQL schema
    pub fn schema_builder(self) -> SchemaBuilder {
        // register queries
        let query: Object = Object::new("Query");
        let query = self
            .queries
            .into_iter()
            .fold(query, |query, field| query.field(field));

        // register mutations
        let mutation = Object::new("Mutation");
        let mutation = self.mutations
            .into_iter()
            .fold(mutation, |mutation, field| mutation.field(field));

        let schema = Schema::build(query.type_name(), Some(mutation.type_name()), None);

        // register output types to schema
        let schema = self
            .outputs
            .into_iter()
            .fold(schema, |schema, entity| schema.register(entity));

        // register input types to schema
        let schema = self
            .inputs
            .into_iter()
            .fold(schema, |schema, edge| schema.register(edge));

        // register enumerations
        let schema = self
            .enumerations
            .into_iter()
            .fold(schema, |schema, enumeration| schema.register(enumeration));

        // register static filter types
        let filter_input_builder = FilterInputBuilder {
            context: self.context,
        };
        schema
            .register(filter_input_builder.string_filter())
            .register(filter_input_builder.integer_filter())
            .register(filter_input_builder.float_filter())
            .register(filter_input_builder.text_filter())
            .register(filter_input_builder.boolean_filter())
            .register(
                OrderByEnumBuilder {
                    context: self.context,
                }
                .enumeration(),
            )
            .register(
                CursorInputBuilder {
                    context: self.context,
                }
                .input_object(),
            )
            .register(
                CursorInputBuilder {
                    context: self.context,
                }
                .input_object(),
            )
            .register(
                PageInputBuilder {
                    context: self.context,
                }
                .input_object(),
            )
            .register(
                OffsetInputBuilder {
                    context: self.context,
                }
                .input_object(),
            )
            .register(
                PaginationInputBuilder {
                    context: self.context,
                }
                .input_object(),
            )
            .register(
                PageInfoObjectBuilder {
                    context: self.context,
                }
                .to_object(),
            )
            .register(
                PaginationInfoObjectBuilder {
                    context: self.context,
                }
                .to_object(),
            )
            .register(query)
            .register(mutation)
    }
}

pub trait RelationBuilder {
    fn get_relation(
        &self,
        context: &'static crate::BuilderContext,
    ) -> async_graphql::dynamic::Field;
}

#[macro_export]
macro_rules! register_entity {
    ($builder:expr, $module_path:ident) => {
        $builder.register_entity::<$module_path::Entity>(
            <$module_path::RelatedEntity as sea_orm::Iterable>::iter()
                .map(|rel| seaography::RelationBuilder::get_relation(&rel, $builder.context))
                .collect(),
        );
    };
}

#[macro_export]
macro_rules! register_entities {
    ($builder:expr, [$($module_paths:ident),+ $(,)?]) => {
        $(seaography::register_entity!($builder, $module_paths);)*
    };
}

#[macro_export]
macro_rules! register_entity_without_relation {
    ($builder:expr, $module_path:ident) => {
        $builder.register_entity::<$module_path::Entity>(vec![]);
    };
}

#[macro_export]
macro_rules! register_entities_without_relation {
    ($builder:expr, [$($module_paths:ident),+ $(,)?]) => {
        $(seaography::register_entity_without_relation!($builder, $module_paths);)*
    };
}
