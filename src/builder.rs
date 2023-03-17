use async_graphql::dynamic::{Field, InputObject, Object, Schema, SchemaBuilder};
use sea_orm::{EntityTrait};
use std::collections::BTreeMap;

use crate::{
    BuilderContext, ConnectionObjectBuilder, EdgeObjectBuilder, EntityObjectBuilder,
    EntityQueryFieldBuilder, FilterInputBuilder, OrderInputBuilder, OrderByEnumBuilder, CursorInputBuilder, PageInputBuilder, OffsetInputBuilder, PaginationInputBuilder, PageInfoObjectBuilder, PaginationInfoObjectBuilder,
};

pub struct Builder {
    pub entities: Vec<Object>,
    pub edges: Vec<Object>,
    pub connections: Vec<Object>,
    pub filters: Vec<InputObject>,
    pub orders: Vec<InputObject>,
    pub queries: Vec<Field>,
    pub relations: BTreeMap<String, Vec<Field>>,
    pub context: &'static BuilderContext,
}

impl Builder {
    pub fn new(context: &'static BuilderContext) -> Self {
        Self {
            entities: Vec::new(),
            edges: Vec::new(),
            connections: Vec::new(),
            filters: Vec::new(),
            orders: Vec::new(),
            queries: Vec::new(),
            relations: BTreeMap::new(),
            context,
        }
    }

    pub fn register_entity<T>(mut self, relations: Vec<Field>) -> Self
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: &self.context,
        };

        let edge_object_builder = EdgeObjectBuilder {
            context: &self.context,
        };

        let connection_object_builder = ConnectionObjectBuilder {
            context: &self.context,
        };

        let filter_input_builder = FilterInputBuilder {
            context: &self.context,
        };

        let order_input_builder = OrderInputBuilder {
            context: &self.context,
        };

        let entity_query_field_builder = EntityQueryFieldBuilder {
            context: &self.context,
        };

        let entity_object = entity_object_builder.to_object::<T>();

        let entity_object = relations.into_iter().fold(entity_object, |entity_object, field| entity_object.field(field));

        self.entities
            .extend(vec![entity_object]);

        let edge = edge_object_builder.to_object::<T>();
        self.edges.extend(vec![edge]);

        let connection = connection_object_builder.to_object::<T>();
        self.connections.extend(vec![connection]);

        let filter = filter_input_builder.to_object::<T>();
        self.filters.extend(vec![filter]);

        let order = order_input_builder.to_object::<T>();
        self.orders.extend(vec![order]);

        let query = entity_query_field_builder.to_field::<T>();
        self.queries.extend(vec![query]);

        self
    }

    pub fn schema_builder(self) -> SchemaBuilder {
        let query = Object::new("Query");

        let query = self
            .queries
            .into_iter()
            .fold(query, |query, field| query.field(field));

        let schema = Schema::build(query.type_name(), None, None);

        let mut relations = self.relations;

        // register entities to schema
        let schema = self
            .entities
            .into_iter()
            // add related fields to entities
            .map(
                |entity: Object| match relations.remove(entity.type_name()) {
                    Some(fields) => fields
                        .into_iter()
                        .fold(entity, |entity, field| entity.field(field)),
                    None => entity,
                },
            )
            .fold(schema, |schema, entity| schema.register(entity));

        // register edges to schema
        let schema = self
            .edges
            .into_iter()
            .fold(schema, |schema, edge| schema.register(edge));

        // register connections to schema
        let schema = self
            .connections
            .into_iter()
            .fold(schema, |schema, connection| schema.register(connection));

        // register filters to schema
        let schema = self
            .filters
            .into_iter()
            .fold(schema, |schema, filter| schema.register(filter));

        // register orders to schema
        let schema = self
            .orders
            .into_iter()
            .fold(schema, |schema, filter| schema.register(filter));

        let filter_input_builder = FilterInputBuilder { context: self.context };

        schema
            .register(filter_input_builder.string_filter())
            .register(filter_input_builder.integer_filter())
            .register(filter_input_builder.float_filter())
            .register(filter_input_builder.text_filter())
            .register(filter_input_builder.boolean_filter())
            .register(OrderByEnumBuilder{ context: self.context }.enumeration())
            .register(CursorInputBuilder{ context: self.context }.input_object())
            .register(CursorInputBuilder{ context: self.context }.input_object())
            .register(PageInputBuilder{ context: self.context }.input_object())
            .register(OffsetInputBuilder{ context: self.context }.input_object())
            .register(PaginationInputBuilder{ context: self.context }.input_object())
            .register(PageInfoObjectBuilder{ context: self.context }.to_object())
            .register(PaginationInfoObjectBuilder{ context: self.context }.to_object())
            .register(query)
    }
}
