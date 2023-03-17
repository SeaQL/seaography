use crate::{
    ActiveEnumConfig, ActiveEnumFilterInputConfig, ConnectionObjectConfig, CursorInputConfig,
    EdgeObjectConfig, EntityQueryFieldConfig, FilterInputConfig, OffsetInputConfig,
    OrderByEnumConfig, OrderInputConfig, PageInfoObjectConfig, PageInputConfig,
    PaginationInfoObjectConfig, PaginationInputConfig,
};

#[derive(Clone, Debug, Default)]
pub struct BuilderContext {
    pub order_by_enum: OrderByEnumConfig,
    pub active_enum: ActiveEnumConfig,

    pub cursor_input: CursorInputConfig,
    pub page_input: PageInputConfig,
    pub offset_input: OffsetInputConfig,
    pub pagination_input: PaginationInputConfig,

    pub order_input: OrderInputConfig,

    pub filter_input: FilterInputConfig,
    pub active_enum_filter_input: ActiveEnumFilterInputConfig,

    pub page_info_object: PageInfoObjectConfig,
    pub pagination_info_object: PaginationInfoObjectConfig,
    pub edge_object: EdgeObjectConfig,
    pub connection_object: ConnectionObjectConfig,
    pub entity_query_field: EntityQueryFieldConfig,
    // guards functions
    // is_skipped function
    // naming function
    // fields type overrides
}
