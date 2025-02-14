use async_graphql::{
    dataloader::DataLoader,
    dynamic::{Enum, Field, FieldFuture, InputObject, Object, Schema, SchemaBuilder, TypeRef},
};
use sea_orm::{ActiveEnum, ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel};

use crate::{
    ActiveEnumBuilder, ActiveEnumFilterInputBuilder, BuilderContext, ConnectionObjectBuilder,
    CursorInputBuilder, EdgeObjectBuilder, EntityCreateBatchMutationBuilder,
    EntityCreateOneMutationBuilder, EntityDeleteMutationBuilder, EntityInputBuilder,
    EntityObjectBuilder, EntityQueryFieldBuilder, EntityUpdateMutationBuilder, FilterInputBuilder,
    FilterTypesMapHelper, OffsetInputBuilder, OneToManyLoader, OneToOneLoader, OrderByEnumBuilder,
    OrderInputBuilder, PageInfoObjectBuilder, PageInputBuilder, PaginationInfoObjectBuilder,
    PaginationInputBuilder,
};

/// The Builder is used to create the Schema for GraphQL
///
/// You can populate it with the entities, enumerations of your choice
pub struct Builder {
    pub query: Object,
    pub mutation: Object,
    pub schema: SchemaBuilder,

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

    /// holds all entities metadata
    pub metadata: std::collections::HashMap<String, serde_json::Value>,

    /// holds a copy to the database connection
    pub connection: sea_orm::DatabaseConnection,

    /// configuration for builder
    pub context: &'static BuilderContext,

    /// Set the maximum depth a query can have
    pub depth: Option<usize>,

    /// Set the maximum complexity a query can have
    pub complexity: Option<usize>,
}

impl Builder {
    /// Used to create a new Builder from the given configuration context
    pub fn new(context: &'static BuilderContext, connection: sea_orm::DatabaseConnection) -> Self {
        let query: Object = Object::new("Query");
        let mutation = Object::new("Mutation").field(Field::new(
            "_ping",
            TypeRef::named(TypeRef::STRING),
            |_| FieldFuture::new(async move { Ok(Some(async_graphql::Value::from("pong"))) }),
        ));
        let schema = Schema::build(query.type_name(), Some(mutation.type_name()), None);

        Self {
            query,
            mutation,
            schema,
            outputs: Vec::new(),
            inputs: Vec::new(),
            enumerations: Vec::new(),
            queries: Vec::new(),
            mutations: Vec::new(),
            metadata: Default::default(),
            connection,
            context,
            depth: None,
            complexity: None,
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
        let entity_object = relations.into_iter().fold(
            entity_object_builder.to_object::<T>(),
            |entity_object, field| entity_object.field(field),
        );

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

        let schema = sea_orm::Schema::new(self.connection.get_database_backend());
        let metadata = schema.json_schema_from_entity(T::default());
        self.metadata.insert(T::default().to_string(), metadata);
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

        let entity_input_builder = EntityInputBuilder {
            context: self.context,
        };

        let entity_insert_input_object = entity_input_builder.insert_input_object::<T>();
        let entity_update_input_object = entity_input_builder.update_input_object::<T>();
        self.inputs
            .extend(vec![entity_insert_input_object, entity_update_input_object]);

        // create one mutation
        let entity_create_one_mutation_builder = EntityCreateOneMutationBuilder {
            context: self.context,
        };
        let create_one_mutation = entity_create_one_mutation_builder.to_field::<T, A>();
        self.mutations.push(create_one_mutation);

        // create batch mutation
        let entity_create_batch_mutation_builder: EntityCreateBatchMutationBuilder =
            EntityCreateBatchMutationBuilder {
                context: self.context,
            };
        let create_batch_mutation = entity_create_batch_mutation_builder.to_field::<T, A>();
        self.mutations.push(create_batch_mutation);

        // update mutation
        let entity_update_mutation_builder = EntityUpdateMutationBuilder {
            context: self.context,
        };
        let update_mutation = entity_update_mutation_builder.to_field::<T, A>();
        self.mutations.push(update_mutation);

        let entity_delete_mutation_builder = EntityDeleteMutationBuilder {
            context: self.context,
        };
        let delete_mutation = entity_delete_mutation_builder.to_field::<T, A>();
        self.mutations.push(delete_mutation);
    }

    pub fn register_entity_dataloader_one_to_one<T, R, S>(mut self, _entity: T, spawner: S) -> Self
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        S: Fn(async_graphql::futures_util::future::BoxFuture<'static, ()>) -> R
            + Send
            + Sync
            + 'static,
    {
        self.schema = self.schema.data(DataLoader::new(
            OneToOneLoader::<T>::new(self.connection.clone()),
            spawner,
        ));
        self
    }

    pub fn register_entity_dataloader_one_to_many<T, R, S>(mut self, _entity: T, spawner: S) -> Self
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        S: Fn(async_graphql::futures_util::future::BoxFuture<'static, ()>) -> R
            + Send
            + Sync
            + 'static,
    {
        self.schema = self.schema.data(DataLoader::new(
            OneToManyLoader::<T>::new(self.connection.clone()),
            spawner,
        ));
        self
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
        let filter_types_map_helper = FilterTypesMapHelper {
            context: self.context,
        };

        let enumeration = active_enum_builder.enumeration::<A>();
        self.enumerations.push(enumeration);

        let filter_info = active_enum_filter_input_builder.filter_info::<A>();
        self.inputs
            .push(filter_types_map_helper.generate_filter_input(&filter_info));
    }

    pub fn set_depth_limit(mut self, depth: Option<usize>) -> Self {
        self.depth = depth;
        self
    }

    pub fn set_complexity_limit(mut self, complexity: Option<usize>) -> Self {
        self.complexity = complexity;
        self
    }

    /// used to consume the builder context and generate a ready to be completed GraphQL schema
    pub fn schema_builder(self) -> SchemaBuilder {
        let query = self.query;
        let mutation = self.mutation;
        let schema = self.schema;

        // register queries
        let query = self
            .queries
            .into_iter()
            .fold(query, |query, field| query.field(field));

        const TABLE_NAME: &str = "table_name";
        let field = Field::new(
            "_sea_orm_entity_metadata",
            TypeRef::named("String"),
            move |ctx| {
                let metadata_hashmap = self.metadata.clone();
                FieldFuture::new(async move {
                    let table_name = ctx
                        .args
                        .get(TABLE_NAME)
                        .expect("table_name is required")
                        .string()?;
                    if let Some(metadata) = metadata_hashmap.get(table_name) {
                        Ok(Some(async_graphql::Value::from_json(metadata.to_owned())?))
                    } else {
                        Ok(None)
                    }
                })
            },
        )
        .argument(async_graphql::dynamic::InputValue::new(
            TABLE_NAME,
            TypeRef::named_nn(TypeRef::STRING),
        ));
        let query = query.field(field);

        // register mutations
        let mutation = self
            .mutations
            .into_iter()
            .fold(mutation, |mutation, field| mutation.field(field));

        // register entities to schema
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

        // register input filters
        let filter_types_map_helper = FilterTypesMapHelper {
            context: self.context,
        };
        let schema = filter_types_map_helper
            .get_input_filters()
            .into_iter()
            .fold(schema, |schema, cur| schema.register(cur));

        let schema = schema
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
            .register(mutation);

        let schema = if let Some(depth) = self.depth {
            schema.limit_depth(depth)
        } else {
            schema
        };
        if let Some(complexity) = self.complexity {
            schema.limit_complexity(complexity)
        } else {
            schema
        }
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
        $builder =
            $builder.register_entity_dataloader_one_to_one($module_path::Entity, tokio::spawn);
        $builder =
            $builder.register_entity_dataloader_one_to_many($module_path::Entity, tokio::spawn);
        $builder.register_entity_mutations::<$module_path::Entity, $module_path::ActiveModel>();
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

#[macro_export]
macro_rules! register_entity_modules {
    ([$($module_paths:ident),+ $(,)?]) => {
        pub fn register_entity_modules(mut builder: seaography::builder::Builder) -> seaography::builder::Builder {
            seaography::register_entities!(
                builder,
                [
                    $($module_paths,)*
                ]
            );
            builder
        }
    };
}

#[macro_export]
macro_rules! register_active_enums {
    ([$($enum_paths:path),+ $(,)?]) => {
        pub fn register_active_enums(mut builder: seaography::builder::Builder) -> seaography::builder::Builder {
            $(builder.register_enumeration::<$enum_paths>();)*
            builder
        }
    };
}
