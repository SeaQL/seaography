use crate::OrmDataloader;
use async_graphql::{dataloader::DataLoader, dynamic::*};
use sea_orm::{DatabaseConnection, RelationTrait};
use seaography::{entity_object_relation, entity_object_via_relation, DynamicGraphqlEntity};

pub fn schema(
    database: DatabaseConnection,
    orm_dataloader: DataLoader<OrmDataloader>,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let order_by_enum = seaography::get_order_by_enum();
    let cursor_input = seaography::get_cursor_input();
    let page_input = seaography::get_page_input();
    let offset_input = seaography::get_offset_input();
    let pagination_input =
        seaography::get_pagination_input(&cursor_input, &page_input, &offset_input);
    let query = Object::new("Query");
    let entities = vec![
        DynamicGraphqlEntity::from_entity::<crate::entities::film_actor::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::film_actor::Entity,
                    crate::entities::actor::Entity,
                >("Actor", crate::entities::film_actor::Relation::Actor.def()),
                entity_object_relation::<
                    crate::entities::film_actor::Entity,
                    crate::entities::film::Entity,
                >("Film", crate::entities::film_actor::Relation::Film.def()),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::rental::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::rental::Entity,
                    crate::entities::customer::Entity,
                >(
                    "Customer",
                    crate::entities::rental::Relation::Customer.def(),
                ),
                entity_object_relation::<
                    crate::entities::rental::Entity,
                    crate::entities::inventory::Entity,
                >(
                    "Inventory",
                    crate::entities::rental::Relation::Inventory.def(),
                ),
                entity_object_relation::<
                    crate::entities::rental::Entity,
                    crate::entities::payment::Entity,
                >("Payment", crate::entities::rental::Relation::Payment.def()),
                entity_object_relation::<
                    crate::entities::rental::Entity,
                    crate::entities::staff::Entity,
                >("Staff", crate::entities::rental::Relation::Staff.def()),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::category::Entity>(
            &pagination_input,
            vec![entity_object_via_relation::<
                crate::entities::category::Entity,
                crate::entities::film::Entity,
            >("Film")],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::staff::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::staff::Entity,
                    crate::entities::address::Entity,
                >("Address", crate::entities::staff::Relation::Address.def()),
                entity_object_relation::<
                    crate::entities::staff::Entity,
                    crate::entities::payment::Entity,
                >("Payment", crate::entities::staff::Relation::Payment.def()),
                entity_object_relation::<
                    crate::entities::staff::Entity,
                    crate::entities::rental::Entity,
                >("Rental", crate::entities::staff::Relation::Rental.def()),
                entity_object_relation::<
                    crate::entities::staff::Entity,
                    crate::entities::staff::Entity,
                >("SelfRef", crate::entities::staff::Relation::SelfRef.def()),
                entity_object_relation::<
                    crate::entities::staff::Entity,
                    crate::entities::staff::Entity,
                >(
                    "SelfRefReverse",
                    crate::entities::staff::Relation::SelfRef.def().rev(),
                ),
                entity_object_relation::<
                    crate::entities::staff::Entity,
                    crate::entities::store::Entity,
                >("Store", crate::entities::staff::Relation::Store.def()),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::country::Entity>(
            &pagination_input,
            vec![entity_object_relation::<
                crate::entities::country::Entity,
                crate::entities::city::Entity,
            >(
                "City", crate::entities::country::Relation::City.def()
            )],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::film::Entity>(
            &pagination_input,
            vec![
                entity_object_via_relation::<
                    crate::entities::film::Entity,
                    crate::entities::actor::Entity,
                >("Actor"),
                entity_object_via_relation::<
                    crate::entities::film::Entity,
                    crate::entities::category::Entity,
                >("Category"),
                entity_object_relation::<
                    crate::entities::film::Entity,
                    crate::entities::inventory::Entity,
                >(
                    "Inventory",
                    crate::entities::film::Relation::Inventory.def(),
                ),
                entity_object_relation::<
                    crate::entities::film::Entity,
                    crate::entities::language::Entity,
                >(
                    "Language1",
                    crate::entities::film::Relation::Language1.def(),
                ),
                entity_object_relation::<
                    crate::entities::film::Entity,
                    crate::entities::language::Entity,
                >(
                    "Language2",
                    crate::entities::film::Relation::Language2.def(),
                ),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::actor::Entity>(
            &pagination_input,
            vec![entity_object_via_relation::<
                crate::entities::actor::Entity,
                crate::entities::film::Entity,
            >("Film")],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::language::Entity>(
            &pagination_input,
            vec![],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::city::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::city::Entity,
                    crate::entities::address::Entity,
                >("Address", crate::entities::city::Relation::Address.def()),
                entity_object_relation::<
                    crate::entities::city::Entity,
                    crate::entities::country::Entity,
                >("Country", crate::entities::city::Relation::Country.def()),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::inventory::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::inventory::Entity,
                    crate::entities::film::Entity,
                >("Film", crate::entities::inventory::Relation::Film.def()),
                entity_object_relation::<
                    crate::entities::inventory::Entity,
                    crate::entities::rental::Entity,
                >("Rental", crate::entities::inventory::Relation::Rental.def()),
                entity_object_relation::<
                    crate::entities::inventory::Entity,
                    crate::entities::store::Entity,
                >("Store", crate::entities::inventory::Relation::Store.def()),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::film_text::Entity>(
            &pagination_input,
            vec![],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::film_category::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::film_category::Entity,
                    crate::entities::category::Entity,
                >(
                    "Category",
                    crate::entities::film_category::Relation::Category.def(),
                ),
                entity_object_relation::<
                    crate::entities::film_category::Entity,
                    crate::entities::film::Entity,
                >("Film", crate::entities::film_category::Relation::Film.def()),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::customer::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::customer::Entity,
                    crate::entities::address::Entity,
                >(
                    "Address",
                    crate::entities::customer::Relation::Address.def(),
                ),
                entity_object_relation::<
                    crate::entities::customer::Entity,
                    crate::entities::payment::Entity,
                >(
                    "Payment",
                    crate::entities::customer::Relation::Payment.def(),
                ),
                entity_object_relation::<
                    crate::entities::customer::Entity,
                    crate::entities::rental::Entity,
                >("Rental", crate::entities::customer::Relation::Rental.def()),
                entity_object_relation::<
                    crate::entities::customer::Entity,
                    crate::entities::store::Entity,
                >("Store", crate::entities::customer::Relation::Store.def()),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::store::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::store::Entity,
                    crate::entities::address::Entity,
                >("Address", crate::entities::store::Relation::Address.def()),
                entity_object_relation::<
                    crate::entities::store::Entity,
                    crate::entities::customer::Entity,
                >("Customer", crate::entities::store::Relation::Customer.def()),
                entity_object_relation::<
                    crate::entities::store::Entity,
                    crate::entities::inventory::Entity,
                >(
                    "Inventory",
                    crate::entities::store::Relation::Inventory.def(),
                ),
                entity_object_relation::<
                    crate::entities::store::Entity,
                    crate::entities::staff::Entity,
                >("Staff", crate::entities::store::Relation::Staff.def()),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::payment::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::payment::Entity,
                    crate::entities::customer::Entity,
                >(
                    "Customer",
                    crate::entities::payment::Relation::Customer.def(),
                ),
                entity_object_relation::<
                    crate::entities::payment::Entity,
                    crate::entities::rental::Entity,
                >("Rental", crate::entities::payment::Relation::Rental.def()),
                entity_object_relation::<
                    crate::entities::payment::Entity,
                    crate::entities::staff::Entity,
                >("Staff", crate::entities::payment::Relation::Staff.def()),
            ],
        ),
        DynamicGraphqlEntity::from_entity::<crate::entities::address::Entity>(
            &pagination_input,
            vec![
                entity_object_relation::<
                    crate::entities::address::Entity,
                    crate::entities::city::Entity,
                >("City", crate::entities::address::Relation::City.def()),
                entity_object_relation::<
                    crate::entities::address::Entity,
                    crate::entities::customer::Entity,
                >(
                    "Customer",
                    crate::entities::address::Relation::Customer.def(),
                ),
                entity_object_relation::<
                    crate::entities::address::Entity,
                    crate::entities::staff::Entity,
                >("Staff", crate::entities::address::Relation::Staff.def()),
                entity_object_relation::<
                    crate::entities::address::Entity,
                    crate::entities::store::Entity,
                >("Store", crate::entities::address::Relation::Store.def()),
            ],
        ),
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
        .register(offset_input)
        .register(pagination_input)
        .register(order_by_enum)
        .register(query)
        .data(database)
        .data(orm_dataloader)
        .finish()
}
