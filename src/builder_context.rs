use crate::{
    ActiveEnumConfig, ActiveEnumFilterInputConfig, ConnectionObjectConfig, CursorInputConfig,
    EdgeObjectConfig, EntityCreateBatchMutationConfig, EntityCreateOneMutationConfig,
    EntityDeleteMutationConfig, EntityInputConfig, EntityObjectConfig, EntityQueryFieldConfig,
    EntityUpdateMutationConfig, FilterInputConfig, HavingInputConfig, OffsetInputConfig,
    OrderByEnumConfig, OrderInputConfig, PageInfoObjectConfig, PageInputConfig,
    PaginationInfoObjectConfig, PaginationInputConfig,
};

pub mod entity_column_id;
pub use entity_column_id::*;

pub mod guards;
pub use guards::*;

pub mod hooks;
pub use hooks::*;

pub mod types_map;
pub use types_map::*;

pub mod filter_types_map;
pub use filter_types_map::*;

/// Used to hold the configuration for various aspects
/// related to our builder options. You can modify the
/// context to make the generated GraphQL nodes match
/// your needs.
#[derive(Default)]
pub struct BuilderContext {
    pub order_by_enum: OrderByEnumConfig,
    pub active_enum: ActiveEnumConfig,

    pub cursor_input: CursorInputConfig,
    pub page_input: PageInputConfig,
    pub offset_input: OffsetInputConfig,
    pub pagination_input: PaginationInputConfig,

    pub order_input: OrderInputConfig,

    pub filter_input: FilterInputConfig,
    pub having_input: HavingInputConfig,
    pub active_enum_filter_input: ActiveEnumFilterInputConfig,

    pub page_info_object: PageInfoObjectConfig,
    pub pagination_info_object: PaginationInfoObjectConfig,
    pub edge_object: EdgeObjectConfig,
    pub entity_object: EntityObjectConfig,
    pub connection_object: ConnectionObjectConfig,
    pub entity_query_field: EntityQueryFieldConfig,

    pub entity_create_one_mutation: EntityCreateOneMutationConfig,
    pub entity_create_batch_mutation: EntityCreateBatchMutationConfig,
    pub entity_update_mutation: EntityUpdateMutationConfig,
    pub entity_delete_mutation: EntityDeleteMutationConfig,

    pub entity_input: EntityInputConfig,

    pub hooks: LifecycleHooks,
    pub types: TypesMapConfig,
    pub filter_types: FilterTypesMapConfig,
}
