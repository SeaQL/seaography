use crate::{OffsetInput, PageInput, PaginationInput};
use async_graphql::dynamic::ValueAccessor;

pub fn get_first(first: Option<ValueAccessor>, pagination: PaginationInput) -> PaginationInput {
    match first {
        Some(first_value) => match first_value.u64() {
            Ok(first_num) => {
                if let Some(offset) = pagination.offset {
                    PaginationInput {
                        offset: Some(OffsetInput {
                            offset: offset.offset,
                            limit: first_num,
                        }),
                        page: None,
                        cursor: None,
                    }
                } else if let Some(page) = pagination.page {
                    PaginationInput {
                        offset: None,
                        page: Some(PageInput {
                            page: page.page,
                            limit: first_num,
                        }),
                        cursor: None,
                    }
                } else {
                    PaginationInput {
                        offset: Some(OffsetInput {
                            offset: 0,
                            limit: first_num,
                        }),
                        page: None,
                        cursor: None,
                    }
                }
            }
            _error => pagination,
        },
        None => pagination,
    }
}