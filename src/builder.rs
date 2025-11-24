use async_graphql::{
    dataloader::DataLoader,
    dynamic::{
        Enum, Field, FieldFuture, InputObject, Object, Scalar, Schema, SchemaBuilder, Subscription,
        SubscriptionField, TypeRef, Union,
    },
};
use sea_orm::{ActiveEnum, ActiveModelTrait, EntityTrait, IntoActiveModel};

use crate::{
    ActiveEnumBuilder, ActiveEnumFilterInputBuilder, BuilderContext, ConnectionObjectBuilder,
    CursorInputBuilder, CustomEnum, CustomFields, CustomInputObject, CustomOutputObject,
    CustomUnion, EdgeObjectBuilder, EntityCreateBatchMutationBuilder,
    EntityCreateOneMutationBuilder, EntityDeleteMutationBuilder, EntityInputBuilder,
    EntityObjectBuilder, EntityQueryFieldBuilder, EntityUpdateMutationBuilder, FilterInputBuilder,
    FilterTypesMapHelper, HavingInputBuilder, OffsetInputBuilder, OneToManyLoader, OneToOneLoader,
    OrderByEnumBuilder, OrderInputBuilder, PageInfoObjectBuilder, PageInputBuilder,
    PaginationInfoObjectBuilder, PaginationInputBuilder, RelatedEntityFilter,
    RelatedEntityFilterField,
};

type MetadataHashMap = std::collections::HashMap<String, serde_json::Value>;

/// The Builder is used to create the Schema for GraphQL
///
/// You can populate it with the entities, enumerations of your choice
pub struct Builder {
    pub query: Object,
    pub mutation: Object,
    pub subscription: Subscription,
    pub schema: SchemaBuilder,

    /// holds all output object types
    pub outputs: Vec<Object>,

    /// holds all input object types
    pub inputs: Vec<InputObject>,

    /// holds all enumeration types
    pub enumerations: Vec<Enum>,

    /// holds all union types
    pub unions: Vec<Union>,

    /// holds all scalar types
    pub scalars: Vec<Scalar>,

    /// holds all entities queries
    pub queries: Vec<Field>,

    /// holds all entities mutations
    pub mutations: Vec<Field>,

    /// holds all subscriptions
    pub subscriptions: Vec<SubscriptionField>,

    /// holds all entities metadata
    pub metadata: MetadataHashMap,

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
            |_| FieldFuture::from_value(Some(async_graphql::Value::from("pong"))),
        ));
        let subscription = Subscription::new("Subscription");

        let schema = Schema::build(
            query.type_name(),
            Some(mutation.type_name()),
            Some(subscription.type_name()),
        );

        Self {
            query,
            mutation,
            subscription,
            schema,
            outputs: Vec::new(),
            inputs: Vec::new(),
            enumerations: Vec::new(),
            unions: Vec::new(),
            scalars: Vec::new(),
            queries: Vec::new(),
            mutations: Vec::new(),
            subscriptions: Vec::new(),
            metadata: Default::default(),
            connection,
            context,
            depth: None,
            complexity: None,
        }
    }

    /// Register a SeaORM Entity to the schema.
    /// With all the edge, connection and filter objects.
    pub fn register_entity<T>(
        &mut self,
        relations: Vec<Field>,
        related_entity_filter: &RelatedEntityFilter<T>,
    ) where
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

        self.outputs.extend([entity_object, edge, connection]);

        let filter_input_builder = FilterInputBuilder {
            context: self.context,
        };
        let filter = filter_input_builder.to_object::<T>();

        let having_input_builder = HavingInputBuilder {
            context: self.context,
        };
        let having = having_input_builder.to_object::<T>(related_entity_filter);

        let order_input_builder = OrderInputBuilder {
            context: self.context,
        };
        let order = order_input_builder.to_object::<T>();

        self.inputs.extend([filter, having, order]);

        let entity_query_field_builder = EntityQueryFieldBuilder {
            context: self.context,
        };

        if cfg!(feature = "field-pluralize") {
            let query = entity_query_field_builder.to_singular_field::<T>();
            self.queries.push(query);
        }

        let connection_query = entity_query_field_builder.to_field::<T>();
        self.queries.push(connection_query);

        let schema = sea_orm::Schema::new(self.connection.get_database_backend());
        let metadata = schema.json_schema_from_entity(T::default());
        self.metadata.insert(T::default().to_string(), metadata);
    }

    /// Register a SeaORM entity to use the Model for input / ouput.
    /// No query / mutation will be added. Intended for use in custom operations.
    pub fn register_custom_entity<T>(&mut self)
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let entity_object = entity_object_builder.to_object::<T>();
        self.outputs.push(entity_object);

        let basic_entity_object = entity_object_builder.to_basic_object::<T>();
        self.outputs.push(basic_entity_object);

        let entity_input_builder = EntityInputBuilder {
            context: self.context,
        };
        let entity_insert_input_object = entity_input_builder.insert_input_object::<T>();
        self.inputs.push(entity_insert_input_object);

        let schema = sea_orm::Schema::new(self.connection.get_database_backend());
        let metadata = schema.json_schema_from_entity(T::default());
        self.metadata.insert(T::default().to_string(), metadata);
    }

    pub fn register_entity_mutations<T, A>(&mut self)
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <T as EntityTrait>::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + Send + 'static,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let basic_entity_object = entity_object_builder.to_basic_object::<T>();
        self.outputs.push(basic_entity_object);

        let entity_input_builder = EntityInputBuilder {
            context: self.context,
        };

        let entity_insert_input_object = entity_input_builder.insert_input_object::<T>();
        let entity_update_input_object = entity_input_builder.update_input_object::<T>();
        self.inputs
            .extend([entity_insert_input_object, entity_update_input_object]);

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

    pub fn register_related_entity_filter<T>(
        mut self,
        related_entity_filter: RelatedEntityFilter<T>,
    ) -> Self
    where
        T: EntityTrait,
    {
        self.schema = self.schema.data(related_entity_filter);
        self
    }

    /// Used to register an SeaORM ActiveEnum to the schema
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

    pub fn register_custom_enum<T>(&mut self)
    where
        T: CustomEnum,
    {
        self.enumerations.push(T::to_enum());
    }

    pub fn register_custom_union<T>(&mut self)
    where
        T: CustomUnion,
    {
        self.unions.push(T::to_union());
    }

    pub fn register_custom_input<T>(&mut self)
    where
        T: CustomInputObject,
    {
        self.inputs.push(T::input_object(self.context));
    }

    /// Register a simple object as custom output
    pub fn register_custom_output<T>(&mut self)
    where
        T: CustomOutputObject,
    {
        self.outputs.push(T::basic_object(self.context));
    }

    /// Register a custom output object without custom fields
    pub fn register_complex_custom_output<T>(&mut self)
    where
        T: CustomOutputObject + CustomFields,
    {
        let object = T::to_fields(self.context)
            .into_iter()
            .fold(T::basic_object(self.context), |object, field| {
                object.field(field)
            });

        self.outputs.push(object);
    }

    pub fn register_custom_object<T>(&mut self)
    where
        T: CustomInputObject + CustomOutputObject,
    {
        self.register_custom_input::<T>();
        self.register_custom_output::<T>();
    }

    pub fn register_custom_query<T>(&mut self)
    where
        T: CustomFields,
    {
        self.queries.append(&mut T::to_fields(self.context));
    }

    pub fn register_custom_mutation<T>(&mut self)
    where
        T: CustomFields,
    {
        self.mutations.append(&mut T::to_fields(self.context));
    }

    pub fn register_subscription_field(&mut self, field: SubscriptionField) {
        self.subscriptions.push(field);
    }

    pub fn register_scalar(&mut self, scalar: Scalar) {
        self.scalars.push(scalar);
    }

    pub fn set_depth_limit(mut self, depth: Option<usize>) -> Self {
        self.depth = depth;
        self
    }

    pub fn set_complexity_limit(mut self, complexity: Option<usize>) -> Self {
        self.complexity = complexity;
        self
    }

    #[cfg(feature = "schema-meta")]
    fn register_schema_meta(&mut self) {
        use crate::schema;

        self.register_custom_output::<schema::Table>();
        self.register_custom_output::<schema::Column>();
        self.register_custom_output::<schema::ColumnType>();
        self.register_custom_output::<schema::Array>();
        self.register_custom_output::<schema::Enumeration>();

        use crate::CustomOutputType;
        use async_graphql::dynamic::FieldValue;

        const TABLE_NAME: &str = "table_name";
        let metadata_hashmap = std::mem::take(&mut self.metadata);

        let field = Field::new(
            "_sea_orm_entity_metadata",
            crate::schema::Table::gql_output_type_ref(self.context),
            move |ctx| {
                let metadata_hashmap = metadata_hashmap.clone();
                FieldFuture::new(async move {
                    let table_name = ctx.args.try_get(TABLE_NAME)?.string()?;
                    if let Some(metadata) = metadata_hashmap.get(table_name) {
                        let table: crate::schema::Table = serde_json::from_value(metadata.clone())?;
                        Ok(Some(FieldValue::owned_any(table)))
                    } else {
                        Err(async_graphql::Error::new(format!(
                            "table not found `{table_name}`"
                        )))
                    }
                })
            },
        )
        .argument(async_graphql::dynamic::InputValue::new(
            TABLE_NAME,
            TypeRef::named_nn(TypeRef::STRING),
        ));

        self.queries.push(field);
    }

    /// Consume the builder context and generate a ready to be completed GraphQL schema.
    /// You can extend the schema, or attach additional data to it before finish().
    pub fn schema_builder(mut self) -> SchemaBuilder {
        #[cfg(feature = "schema-meta")]
        self.register_schema_meta();

        let query = self.query;
        let mutation = self.mutation;
        let subscription = self.subscription;
        let schema = self.schema;
        let have_subscription = !self.subscriptions.is_empty();

        // register queries
        let query = self
            .queries
            .into_iter()
            .fold(query, |query, field| query.field(field));

        // register mutations
        let mutation = self
            .mutations
            .into_iter()
            .fold(mutation, |mutation, field| mutation.field(field));

        // register subscriptions
        let subscription = self
            .subscriptions
            .into_iter()
            .fold(subscription, |subscription, field| {
                subscription.field(field)
            });

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

        // register unions
        let schema = self
            .unions
            .into_iter()
            .fold(schema, |schema, enumeration| schema.register(enumeration));

        // register scalars
        let schema = self
            .scalars
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

        let json_scalar = Scalar::new("Json");

        let schema = schema
            .register(json_scalar)
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

        let schema = if have_subscription {
            schema.register(subscription)
        } else {
            schema
        };

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
    fn get_relation_name(&self, context: &'static BuilderContext) -> String;

    fn get_relation(&self, context: &'static BuilderContext) -> async_graphql::dynamic::Field;

    fn get_related_entity_filter(
        &self,
        context: &'static BuilderContext,
    ) -> RelatedEntityFilterField;
}

#[macro_export]
macro_rules! impl_custom_type_for_enum {
    ($name:ty) => {
        impl seaography::CustomOutputType for $name {
            fn gql_output_type_ref(
                ctx: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::TypeRef {
                <$name as seaography::GqlScalarValueType>::gql_output_type_ref(ctx)
            }

            fn gql_field_value(
                self,
                ctx: &'static seaography::BuilderContext,
            ) -> Option<async_graphql::dynamic::FieldValue<'static>> {
                <$name as seaography::GqlScalarValueType>::gql_field_value(self, ctx)
            }
        }

        impl seaography::CustomInputType for $name {
            fn gql_input_type_ref(
                ctx: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::TypeRef {
                <$name as seaography::GqlScalarValueType>::gql_input_type_ref(ctx)
            }

            fn parse_value(
                context: &'static seaography::BuilderContext,
                value: Option<async_graphql::dynamic::ValueAccessor<'_>>,
            ) -> seaography::SeaResult<Self> {
                match value {
                    None => Err(seaography::SeaographyError::AsyncGraphQLError(
                        "Value expected".into(),
                    )),
                    Some(v) => {
                        let s = v.enum_name()?.to_string();
                        match sea_orm::ActiveEnum::try_from_value(&s) {
                            Ok(v) => Ok(v),
                            Err(e) => Err(seaography::SeaographyError::AsyncGraphQLError(e.into())),
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_custom_output_type_for_entity {
    ($name:ty) => {
        #[allow(non_local_definitions)]
        impl seaography::CustomOutputType for $name {
            fn gql_output_type_ref(
                ctx: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::TypeRef {
                <$name as seaography::GqlModelType>::gql_output_type_ref(ctx)
            }

            fn gql_field_value(
                self,
                ctx: &'static seaography::BuilderContext,
            ) -> Option<async_graphql::dynamic::FieldValue<'static>> {
                <$name as seaography::GqlModelType>::gql_field_value(self, ctx)
            }
        }
    };
}

#[macro_export]
/// Includes mutations by default. Can skip mutations by
/// `seaography::register_entity!(builder, my_entity, mutation: false);`.
macro_rules! register_entity {
    ($builder:expr, $module_path:ident) => {
        seaography::register_entity!($builder, $module_path, mutation: true);
    };
    ($builder:expr, $module_path:ident, mutation: $mutation:expr) => {
        let relations =
            <$module_path::RelatedEntity as sea_orm::Iterable>::iter()
                .map(|rel| seaography::RelationBuilder::get_relation(&rel, $builder.context))
                .collect();
        let related_entity_filter =
            seaography::RelatedEntityFilter::<$module_path::Entity>::build::<$module_path::RelatedEntity>($builder.context);

        $builder.register_entity::<$module_path::Entity>(relations, &related_entity_filter);
        $builder =
            $builder.register_entity_dataloader_one_to_one($module_path::Entity, tokio::spawn);
        $builder =
            $builder.register_entity_dataloader_one_to_many($module_path::Entity, tokio::spawn);
        $builder =
            $builder.register_related_entity_filter::<$module_path::Entity>(related_entity_filter);
        if $mutation {
            $builder.register_entity_mutations::<$module_path::Entity, $module_path::ActiveModel>();
        }
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
#[cfg(feature = "strict-custom-types")]
macro_rules! impl_custom_output_type_for_entities {
    ([$($module_paths:ident),+ $(,)?]) => {
        $(seaography::impl_custom_output_type_for_entity!($module_paths::Model);)*
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

#[macro_export]
macro_rules! register_custom_entities {
    ($builder:expr, [$($entity:ident),+ $(,)?]) => {
        $($builder.register_custom_entity::<$entity::Entity>();)*
    };
}

#[macro_export]
macro_rules! register_custom_inputs {
    ($builder:expr, [$($ty:path),+ $(,)?]) => {
        $($builder.register_custom_input::<$ty>();)*
    };
}

#[macro_export]
macro_rules! register_custom_outputs {
    ($builder:expr, [$($ty:path),+ $(,)?]) => {
        $($builder.register_custom_output::<$ty>();)*
    };
}

#[macro_export]
macro_rules! register_custom_unions {
    ($builder:expr, [$($ty:path),+ $(,)?]) => {
        $($builder.register_custom_union::<$ty>();)*
    };
}

#[macro_export]
macro_rules! register_complex_custom_outputs {
    ($builder:expr, [$($ty:path),+ $(,)?]) => {
        $($builder.register_complex_custom_output::<$ty>();)*
    };
}

#[macro_export]
macro_rules! register_custom_queries {
    ($builder:expr, [$($ty:path),+ $(,)?]) => {
        $($builder.register_custom_query::<$ty>();)*
    };
}

#[macro_export]
macro_rules! register_custom_mutations {
    ($builder:expr, [$($ty:path),+ $(,)?]) => {
        $($builder.register_custom_mutation::<$ty>();)*
    };
}
