use async_graphql::dynamic::{Schema, SchemaError};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use seaography::{
    Builder, BuilderContext, EntityObjectConfig, EntityQueryFieldConfig, TimeLibrary,
    TypesMapConfig,
    heck::{ToSnakeCase, ToUpperCamelCase},
    lazy_static,
    CustomFields,
};

use crate::{
    backend::Backend,
    entities::{accounts, drawings, objects, projects},
    queries,
    mutations,
    subscriptions,
    types,
};

fn singular(name: impl Into<String>) -> String {
    let mut name = name.into();
    if name.ends_with("ies") {
        name.truncate(name.len() - 3);
        name.push('y');
    } else if name.ends_with('s') {
        name.remove(name.len() - 1);
    }
    name
}

fn type_name(entity_name: &str) -> String {
    singular(entity_name).to_upper_camel_case()
}

fn column_name(_entity_name: &str, column_name: &str) -> String {
    column_name.to_snake_case()
}

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        let context = BuilderContext::default();
        let types = TypesMapConfig::default();
        let types = TypesMapConfig {
            time_library: TimeLibrary::Chrono,
            ..types
        };
        let mut context = BuilderContext {
            // hooks: LifecycleHooks::new(AuthHooks::new()),
            entity_object: EntityObjectConfig {
                type_name: Box::new(type_name),
                column_name: Box::new(column_name),
                basic_type_suffix: "Basic".into(),
            },
            entity_query_field: EntityQueryFieldConfig {
                combine_is_null_is_not_null: true,
                use_ilike: true,
                ..Default::default()
            },
            types,
            ..context
        };

        context.pagination_input.default_limit = Some(100);
        context.pagination_input.max_limit = Some(500);

        // set_column_options(&mut context);

        context
    };
}

macro_rules! register_entity {
    ($builder:expr, $module_path:ident $(, $custom_fields:expr)? ) => {
        seaography::impl_custom_output_type_for_entity!($module_path::Model);

        let fields: Vec<::async_graphql::dynamic::Field> =
            <$module_path::RelatedEntity as sea_orm::Iterable>::iter()
            .map(|rel| seaography::RelationBuilder::get_relation(&rel, $builder.context))
            .collect();
        $(
            let fields = {
                let mut temp = fields;
                temp.extend($custom_fields);
                temp
            };
        )?

        $builder.register_entity::<$module_path::Entity>(fields);
        $builder =
            $builder.register_entity_dataloader_one_to_one($module_path::Entity, tokio::spawn);
        $builder =
            $builder.register_entity_dataloader_one_to_many($module_path::Entity, tokio::spawn);
        // Avoid using the default mutations, since we do updates and deletions on a per-row basis,
        // generate ids on the server side at creation time, set created_at/updated_at fields
        // on the server side to ensure they're accurate, implement soft deletes, and perform
        // validation.
        // $builder.register_entity_mutations::<$module_path::Entity, $module_path::ActiveModel>();
    };
}

pub fn schema(backend: Backend) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, backend.db.clone());

    register_entity!(builder, accounts);
    register_entity!(builder, drawings, drawings::Model::to_fields(&CONTEXT));
    register_entity!(builder, objects, objects::Model::to_fields(&CONTEXT));
    register_entity!(builder, projects);

    builder
        .mutations
        .extend(mutations::CustomMutations::to_fields(&CONTEXT));

    builder
        .queries
        .extend(queries::CustomQueries::to_fields(&CONTEXT));

    builder
        .subscriptions
        .extend(subscriptions::subscriptions(&CONTEXT));

    builder.register_custom_input::<types::Fill>();
    builder.register_custom_input::<types::Stroke>();
    // builder.register_custom_input::<types::Style>();
    builder.register_custom_input::<types::Color>();
    builder.register_custom_input::<types::Point>();
    builder.register_custom_input::<types::Size>();
    builder.register_custom_input::<types::Rectangle>();
    builder.register_custom_input::<types::Circle>();
    builder.register_custom_input::<types::Triangle>();
    // builder.register_custom_input::<types::Shape>();

    builder.register_custom_output::<types::Fill>();
    builder.register_custom_output::<types::Stroke>();
    // builder.register_custom_output::<types::Style>();
    builder.register_custom_output::<types::Color>();
    builder.register_custom_output::<types::Point>();
    builder.register_custom_output::<types::Size>();
    builder.register_custom_output::<types::Rectangle>();
    builder.register_custom_output::<types::Circle>();
    builder.register_custom_output::<types::Triangle>();
    // builder.register_custom_output::<types::Shape>();

    builder.register_custom_enum::<types::Style>();
    builder.register_custom_input::<types::Shape>();
    builder.register_custom_union::<types::Shape>();

    // seaography::register_custom_inputs!(
    //     builder,
    //     [
    //         types::RentalRequest,
    //         types::Location,
    //         types::Point,
    //         types::Size,
    //         types::Rectangle,
    //         types::Circle,
    //         types::Triangle,
    //         types::Shape,
    //     ]
    // );

    // seaography::register_custom_outputs!(
    //     builder,
    //     [
    //         types::PurchaseOrder,
    //         types::Lineitem,
    //         types::ProductSize,
    //         types::Point,
    //         types::Size,
    //     ]
    // );

    // seaography::register_complex_custom_outputs!(
    //     builder,
    //     [types::Rectangle, types::Circle, types::Triangle]
    // );

    // seaography::register_custom_unions!(builder, [types::Shape]);

    // seaography::register_custom_queries!(builder, [queries::Operations]);

    // seaography::register_custom_mutations!(builder, [mutations::Operations]);



    let db = backend.db.clone();
    builder
        .schema_builder()
        .enable_uploading()
        .data(backend)
        .data(db)
        .finish()
}

/// This handler serves the all GraphQL queries and mutations. Before handing
/// control to the query and mutation schemas, it will extract important headers
/// and metadata and insert them into the context.
pub async fn queries_and_mutations(
    schema: Extension<Schema>,
    // auth_provider: Extension<AuthProvider>,
    // headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();
    // if let Some(token) = extract_token_from_headers(&headers) {
    //     match auth_provider.0.validate_token(&token).await {
    //         Ok(claims) => {
    //             let mut retry_count = 0;
    //             loop {
    //                 match auth_provider.get_or_create_user_access(&claims).await {
    //                     Ok(access) => {
    //                         req.data.insert(access);
    //                         break;
    //                     }
    //                     Err(e) => {
    //                         retry_count += 1;
    //                         if retry_count >= 3 {
    //                             tracing::error!("bad auth: internal: {:?}", e);
    //                             break;
    //                         }
    //                         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    //                     }
    //                 }
    //             }
    //         }
    //         Err(e) => tracing::error!("bad auth: {:?}", e),
    //     }
    // }
    schema.execute(req).await.into()
}
