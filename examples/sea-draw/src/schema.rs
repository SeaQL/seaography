use async_graphql::dynamic::{Schema, SchemaError, TypeRef, ValueAccessor};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Extension, http::HeaderMap};
use seaography::{
    Builder, BuilderContext, ColumnOptions, ConvertOutput, CustomFields, EntityColumnId,
    EntityObjectConfig, EntityQueryFieldConfig, TimeLibrary, TypesMapConfig,
    heck::{ToSnakeCase, ToUpperCamelCase},
    lazy_static,
};
use std::{collections::BTreeMap, str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::{
    backend::Backend,
    entities::{
        Access, Account, Permission, accounts, drawings, objects, project_permissions,
        project_permissions::permissions_by_account, projects,
    },
    mutations, queries, subscriptions,
    types::{self, Shape},
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

fn convert_value_from_json(accessor: &ValueAccessor) -> seaography::SeaResult<sea_orm::Value> {
    let value = accessor.as_value();
    let json = value
        .clone()
        .into_json()
        .map_err(|e| seaography::SeaographyError::AsyncGraphQLError(e.to_string().into()))?;
    Ok(sea_orm::Value::from(json))
}

fn convert_output_column_options<T>(
    input_type: Option<TypeRef>,
    output_type: Option<TypeRef>,
) -> ColumnOptions
where
    T: ConvertOutput + 'static,
{
    let mut options = ColumnOptions::default();
    options.input_conversion = Some(Arc::new(convert_value_from_json));
    options.output_conversion = Some(Arc::new(T::convert_output));
    options.input_type = input_type;
    options.output_type = output_type;
    options
}

fn set_column_options(context: &mut BuilderContext) {
    context.types.column_options.insert(
        EntityColumnId::of::<objects::Entity>(&objects::Column::Shape),
        convert_output_column_options::<Shape>(
            Some(TypeRef::named_nn("ShapeInput")),
            Some(TypeRef::named_nn("Shape")),
        ),
    );
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

        set_column_options(&mut context);

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
    register_entity!(builder, project_permissions);

    builder.register_enumeration::<Permission>();

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
    builder.register_complex_custom_output::<types::Rectangle>();
    builder.register_complex_custom_output::<types::Circle>();
    builder.register_complex_custom_output::<types::Triangle>();
    // builder.register_custom_output::<types::Shape>();

    builder.register_custom_enum::<types::Style>();
    builder.register_custom_input::<types::Shape>();
    builder.register_custom_union::<types::Shape>();

    let db = backend.db.clone();
    builder
        .schema_builder()
        .enable_uploading()
        .data(backend)
        .data(db)
        .finish()
}

/// Helper function for extract important headers and metadata.
fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.to_lowercase().starts_with("bearer ") {
                Some(String::from(&value[7..]))
            } else {
                None
            }
        })
}

async fn create_access_object(
    backend: &Backend,
    token: String,
) -> Result<Access, Box<dyn std::error::Error>> {
    let account_id = Uuid::from_str(&token)?;

    let account = backend
        .find_by_id::<Account>(account_id)
        .await?
        .ok_or("Account not found")?;

    let mut permissions: BTreeMap<Uuid, Permission> = BTreeMap::new();

    let project_ps = permissions_by_account(&backend.db, account_id).await?;
    for p in project_ps.iter() {
        permissions.insert(p.project_id, p.permission);
    }
    let is_root = account.id == backend.root_account_id;
    Ok(Access::new(account, permissions, is_root))
}

/// This handler serves the all GraphQL queries and mutations. Before handing
/// control to the query and mutation schemas, it will extract important headers
/// and metadata and insert them into the context.
pub async fn queries_and_mutations(
    schema: Extension<Schema>,
    backend: Extension<Backend>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    if let Some(token) = extract_token_from_headers(&headers) {
        match create_access_object(&backend, token).await {
            Ok(access) => {
                // tracing::info!("queries_and_mutations: account {}", access.account_id());
                req.data.insert(access);
            }
            Err(e) => {
                tracing::error!("bad auth: {:?}", e);
            }
        }
    }
    schema.execute(req).await.into()
}
