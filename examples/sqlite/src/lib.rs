pub mod entities;
pub mod query_root;

macro_rules! entity_custom_type {
    ($name:ty) => {
        impl ::seaography::CustomOutputType for $name {
            fn gql_output_type_ref(
                ctx: &'static ::seaography::BuilderContext,
            ) -> ::async_graphql::dynamic::TypeRef {
                <$name as ::seaography::GqlModelType>::gql_output_type_ref(ctx)
            }

            fn gql_field_value(
                value: Self,
            ) -> Option<::async_graphql::dynamic::FieldValue<'static>> {
                <$name as ::seaography::GqlModelType>::gql_field_value(value)
            }
        }
    };
}

entity_custom_type!(entities::actor::Model);
entity_custom_type!(entities::address::Model);
entity_custom_type!(entities::category::Model);
entity_custom_type!(entities::city::Model);
entity_custom_type!(entities::country::Model);
entity_custom_type!(entities::customer::Model);
entity_custom_type!(entities::film::Model);
entity_custom_type!(entities::film_actor::Model);
entity_custom_type!(entities::film_category::Model);
entity_custom_type!(entities::film_text::Model);
entity_custom_type!(entities::inventory::Model);
entity_custom_type!(entities::language::Model);
entity_custom_type!(entities::payment::Model);
entity_custom_type!(entities::rental::Model);
entity_custom_type!(entities::staff::Model);
entity_custom_type!(entities::store::Model);
