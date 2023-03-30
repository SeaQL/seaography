use std::collections::BTreeMap;

use crate::OrmDataloader;
use async_graphql::{dataloader::DataLoader, dynamic::*};
use sea_orm::{DatabaseConnection, RelationTrait};
use seaography::{
    Builder, BuilderContext, EntityObjectRelationBuilder, EntityObjectViaRelationBuilder, FnGuard, GuardsConfig
};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        let context = BuilderContext::default();
        let mut entity_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        entity_guards.insert("FilmCategory".into(), Box::new(|_ctx| {
            true
        }));
        let mut field_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
        field_guards.insert("Language.lastUpdate".into(), Box::new(|_ctx| {
            true
        }));
        BuilderContext {
            guards: GuardsConfig {
                entity_guards,
                field_guards,
            },
            ..context
        }
    };
}

// lazy_static::lazy_static! { static ref CONTEXT : BuilderContext = BuilderContext :: default () ; }

pub fn schema(
    database: DatabaseConnection,
    orm_dataloader: DataLoader<OrmDataloader>,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT);
    let entity_object_relation_builder = EntityObjectRelationBuilder { context: &CONTEXT };
    let entity_object_via_relation_builder = EntityObjectViaRelationBuilder { context: &CONTEXT };
    builder.register_entity::<crate::entities::film_actor::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::film_actor::Entity, crate::entities::actor::Entity>(
                "actor",
                crate::entities::film_actor::Relation::Actor.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::film_actor::Entity, crate::entities::film::Entity>(
                "film",
                crate::entities::film_actor::Relation::Film.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::rental::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::rental::Entity, crate::entities::customer::Entity>(
                "customer",
                crate::entities::rental::Relation::Customer.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::rental::Entity, crate::entities::inventory::Entity>(
                "inventory",
                crate::entities::rental::Relation::Inventory.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::rental::Entity, crate::entities::payment::Entity>(
                "payment",
                crate::entities::rental::Relation::Payment.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::rental::Entity, crate::entities::staff::Entity>(
                "staff",
                crate::entities::rental::Relation::Staff.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::category::Entity>(vec![
        entity_object_via_relation_builder
            .get_relation::<crate::entities::category::Entity, crate::entities::film::Entity>(
                "film",
            ),
    ]);
    builder.register_entity::<crate::entities::staff::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::staff::Entity, crate::entities::address::Entity>(
                "address",
                crate::entities::staff::Relation::Address.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::staff::Entity, crate::entities::payment::Entity>(
                "payment",
                crate::entities::staff::Relation::Payment.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::staff::Entity, crate::entities::rental::Entity>(
                "rental",
                crate::entities::staff::Relation::Rental.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::staff::Entity, crate::entities::staff::Entity>(
                "selfRef",
                crate::entities::staff::Relation::SelfRef.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::staff::Entity, crate::entities::staff::Entity>(
                "selfRefReverse",
                crate::entities::staff::Relation::SelfRef.def().rev(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::staff::Entity, crate::entities::store::Entity>(
                "store",
                crate::entities::staff::Relation::Store.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::country::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::country::Entity, crate::entities::city::Entity>(
                "city",
                crate::entities::country::Relation::City.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::film::Entity>(vec![
        entity_object_via_relation_builder
            .get_relation::<crate::entities::film::Entity, crate::entities::actor::Entity>("actor"),
        entity_object_via_relation_builder
            .get_relation::<crate::entities::film::Entity, crate::entities::category::Entity>(
                "category",
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::film::Entity, crate::entities::inventory::Entity>(
                "inventory",
                crate::entities::film::Relation::Inventory.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::film::Entity, crate::entities::language::Entity>(
                "language1",
                crate::entities::film::Relation::Language1.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::film::Entity, crate::entities::language::Entity>(
                "language2",
                crate::entities::film::Relation::Language2.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::actor::Entity>(vec![
        entity_object_via_relation_builder
            .get_relation::<crate::entities::actor::Entity, crate::entities::film::Entity>("film"),
    ]);
    builder.register_entity::<crate::entities::language::Entity>(vec![]);
    builder.register_entity::<crate::entities::city::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::city::Entity, crate::entities::address::Entity>(
                "address",
                crate::entities::city::Relation::Address.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::city::Entity, crate::entities::country::Entity>(
                "country",
                crate::entities::city::Relation::Country.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::inventory::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::inventory::Entity, crate::entities::film::Entity>(
                "film",
                crate::entities::inventory::Relation::Film.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::inventory::Entity, crate::entities::rental::Entity>(
                "rental",
                crate::entities::inventory::Relation::Rental.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::inventory::Entity, crate::entities::store::Entity>(
                "store",
                crate::entities::inventory::Relation::Store.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::film_text::Entity>(vec![]);
    builder . register_entity :: < crate :: entities :: film_category :: Entity > (vec ! [entity_object_relation_builder . get_relation :: < crate :: entities :: film_category :: Entity , crate :: entities :: category :: Entity > ("category" , crate :: entities :: film_category :: Relation :: Category . def ()) , entity_object_relation_builder . get_relation :: < crate :: entities :: film_category :: Entity , crate :: entities :: film :: Entity > ("film" , crate :: entities :: film_category :: Relation :: Film . def ())]) ;
    builder.register_entity::<crate::entities::customer::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::customer::Entity, crate::entities::address::Entity>(
                "address",
                crate::entities::customer::Relation::Address.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::customer::Entity, crate::entities::payment::Entity>(
                "payment",
                crate::entities::customer::Relation::Payment.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::customer::Entity, crate::entities::rental::Entity>(
                "rental",
                crate::entities::customer::Relation::Rental.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::customer::Entity, crate::entities::store::Entity>(
                "store",
                crate::entities::customer::Relation::Store.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::store::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::store::Entity, crate::entities::address::Entity>(
                "address",
                crate::entities::store::Relation::Address.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::store::Entity, crate::entities::customer::Entity>(
                "customer",
                crate::entities::store::Relation::Customer.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::store::Entity, crate::entities::inventory::Entity>(
                "inventory",
                crate::entities::store::Relation::Inventory.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::store::Entity, crate::entities::staff::Entity>(
                "staff",
                crate::entities::store::Relation::Staff.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::payment::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::payment::Entity, crate::entities::customer::Entity>(
                "customer",
                crate::entities::payment::Relation::Customer.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::payment::Entity, crate::entities::rental::Entity>(
                "rental",
                crate::entities::payment::Relation::Rental.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::payment::Entity, crate::entities::staff::Entity>(
                "staff",
                crate::entities::payment::Relation::Staff.def(),
            ),
    ]);
    builder.register_entity::<crate::entities::address::Entity>(vec![
        entity_object_relation_builder
            .get_relation::<crate::entities::address::Entity, crate::entities::city::Entity>(
                "city",
                crate::entities::address::Relation::City.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::address::Entity, crate::entities::customer::Entity>(
                "customer",
                crate::entities::address::Relation::Customer.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::address::Entity, crate::entities::staff::Entity>(
                "staff",
                crate::entities::address::Relation::Staff.def(),
            ),
        entity_object_relation_builder
            .get_relation::<crate::entities::address::Entity, crate::entities::store::Entity>(
                "store",
                crate::entities::address::Relation::Store.def(),
            ),
    ]);
    let schema = builder.schema_builder();
    let schema = if let Some(depth) = depth {
        schema.limit_depth(depth)
    } else {
        schema
    };
    let schema = if let Some(complexity) = complexity {
        schema.limit_complexity(complexity)
    } else {
        schema
    };
    schema.data(database).data(orm_dataloader).finish()
}
