use async_graphql::{dataloader::DataLoader, dynamic::*};
use sea_orm::DatabaseConnection;
use seaography::DynamicGraphqlEntity;

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
        DynamicGraphqlEntity::from_entity::<crate::entities::actor::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::address::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::category::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::city::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::country::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::customer::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::film_actor::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::film_category::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::film_text::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::film::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::inventory::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::language::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::payment::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::rental::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::staff::Entity>(&pagination_input),
        DynamicGraphqlEntity::from_entity::<crate::entities::store::Entity>(&pagination_input),
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



// pub fn entity_object_relation<T, R>() -> Field
// where
//     T: EntityTrait,
//     <T as EntityTrait>::Model: Sync,
//     R: EntityTrait,
//     T: Related<R>
// {
//     let relation_definition = <T as Related<R>>::to();

//     let name = format!("{:?}", relation).to_lower_camel_case();

//     let type_name: String =
//         if let sea_orm::sea_query::TableRef::Table(name) = relation_definition.to_tbl {
//             name.to_string()
//         } else {
//             // TODO look this
//             "PANIC!".into()
//         }
//         .to_upper_camel_case();

//     let field = match relation_definition.rel_type {
//         sea_orm::RelationType::HasOne => {
//             Field::new(name, TypeRef::named(format!("{}", type_name)), |ctx| {
//                 // TODO
//                 // dataloader applied here!
//                 let parent: T::Model = ctx.parent_value.try_downcast_ref::<T::Model>()?;

//                 let stmt = <T as Related<relation as EntityTrait>>::find_related();

//                 FieldFuture::new(async move { Ok(Some(Value::Null)) })
//             })
//         }
//         sea_orm::RelationType::HasMany => Field::new(
//             name,
//             TypeRef::named_nn_list_nn(format!("{}Connection", type_name)),
//             |ctx| FieldFuture::new(async move {
//                 // TODO
//                 // each has unique query in order to apply pagination...
//                 Ok(Some(Value::Null))
//             }),
//         ),
//     };

//     field
//         .argument(InputValue::new(
//             "filters",
//             TypeRef::named(format!("{}FilterInput", type_name)),
//         ))
//         .argument(InputValue::new(
//             "orderBy",
//             TypeRef::named(format!("{}OrderInput", type_name)),
//         ))
//         .argument(InputValue::new(
//             "pagination",
//             TypeRef::named("PaginationInput"),
//     ))
// }
