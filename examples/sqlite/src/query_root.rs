use async_graphql::{dataloader::DataLoader, dynamic::*};
use sea_orm::DatabaseConnection;
use seaography::{DynamicGraphqlEntity, entity_object_relation};

use crate::OrmDataloader;

pub fn schema(
    database: DatabaseConnection,
    orm_dataloader: DataLoader<OrmDataloader>,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let order_by_enum = seaography::get_order_by_enum();
    let cursor_input = seaography::get_cursor_input();
    let page_input = seaography::get_page_input();
    let pagination_input = seaography::get_pagination_input(&cursor_input, &page_input);

    let query = Object::new("Query");

    let entities = vec![
        DynamicGraphqlEntity::from_entity::<crate::entities::actor::Entity>(&pagination_input, vec![
            entity_object_relation::<crate::entities::actor::Entity, crate::entities::film_actor::Entity>("filmActors")
        ]),
        DynamicGraphqlEntity::from_entity::<crate::entities::address::Entity>(&pagination_input, vec![
            entity_object_relation::<crate::entities::address::Entity, crate::entities::city::Entity>("city"),
            entity_object_relation::<crate::entities::address::Entity, crate::entities::customer::Entity>("customer"),
            entity_object_relation::<crate::entities::address::Entity, crate::entities::staff::Entity>("staff"),
            entity_object_relation::<crate::entities::address::Entity, crate::entities::store::Entity>("store"),
        ]),
        DynamicGraphqlEntity::from_entity::<crate::entities::category::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::city::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::country::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::customer::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::film_actor::Entity>(&pagination_input, vec![

            entity_object_relation::<crate::entities::film_actor::Entity, crate::entities::film::Entity>("film"),
            entity_object_relation::<crate::entities::film_actor::Entity, crate::entities::actor::Entity>("actor"),
        ]),
        DynamicGraphqlEntity::from_entity::<crate::entities::film_category::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::film_text::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::film::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::inventory::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::language::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::payment::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::rental::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::staff::Entity>(&pagination_input, vec![]),
        DynamicGraphqlEntity::from_entity::<crate::entities::store::Entity>(&pagination_input, vec![]),
    ];

    let schema = Schema::build(query.type_name(), None, None);

    let (schema, query) = entities
        .into_iter()
        .fold((schema, query), |(schema, query), object| {
            (
                schema
                    .register(object.filter_input)
                    .register(object.order_input)
                    .register(object.edge_object)
                    .register(object.connection_object)
                    .register(object.entity_object),
                query.field(object.query),
            )
        });

    let schema = if let Some(depth) = depth {
        schema.limit_depth(depth)
    } else {
        schema
    };

    let schema = seaography::get_filter_types()
        .into_iter()
        .fold(schema, |schema, object| schema.register(object));

    let schema = if let Some(complexity) = complexity {
        schema.limit_complexity(complexity)
    } else {
        schema
    };

    schema
        .register(seaography::PageInfo::to_object())
        .register(seaography::PaginationInfo::to_object())
        .register(cursor_input)
        .register(page_input)
        .register(pagination_input)
        .register(order_by_enum)
        .register(query)
        .data(database)
        .data(orm_dataloader)
        .finish()
}
