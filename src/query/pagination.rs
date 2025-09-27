use itertools::Itertools;
use sea_orm::{
    ConnectionTrait, EntityTrait, Iterable, ModelTrait, PaginatorTrait, PrimaryKeyArity,
    PrimaryKeyToColumn, PrimaryKeyTrait, QuerySelect, QueryTrait, Select,
};

use crate::{
    decode_cursor, encode_cursor, map_cursor_values, BuilderContext, Connection, Edge, PageInfo,
    PageInput, PaginationInfo, PaginationInput,
};

/// used to parse pagination input object and apply it to statement
pub async fn apply_pagination<T, C>(
    context: &'static BuilderContext,
    db: &C,
    stmt: Select<T>,
    pagination: PaginationInput,
) -> Result<Connection<T>, sea_orm::DbErr>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
    C: ConnectionTrait,
{
    let pagination = apply_pagination_defaults(context, pagination);

    if let Some(cursor_object) = pagination.cursor {
        check_limit(context, cursor_object.limit)?;

        let next_stmt = stmt.clone();
        let previous_stmt = stmt.clone();

        fn apply_stmt_cursor_by<T>(
            stmt: sea_orm::entity::prelude::Select<T>,
        ) -> Result<sea_orm::Cursor<sea_orm::SelectModel<T::Model>>, sea_orm::DbErr>
        where
            T: EntityTrait,
            <T as EntityTrait>::Model: Sync,
        {
            let size = <<T::PrimaryKey as PrimaryKeyTrait>::ValueType as PrimaryKeyArity>::ARITY;
            if size == 1 {
                let column = T::PrimaryKey::iter()
                    .map(|variant| variant.into_column())
                    .collect::<Vec<T::Column>>()[0];
                Ok(stmt.cursor_by(column))
            } else if size == 2 {
                let columns = T::PrimaryKey::iter()
                    .map(|variant| variant.into_column())
                    .collect_tuple::<(T::Column, T::Column)>()
                    .expect("infallible as arity is already checked");
                Ok(stmt.cursor_by(columns))
            } else if size == 3 {
                let columns = T::PrimaryKey::iter()
                    .map(|variant| variant.into_column())
                    .collect_tuple::<(T::Column, T::Column, T::Column)>()
                    .expect("infallible as arity is already checked");
                Ok(stmt.cursor_by(columns))
            } else {
                Err(sea_orm::DbErr::Custom(format!(
                    "Not supporting primary key with arity > 3: {size}"
                )))
            }
        }

        let mut stmt = apply_stmt_cursor_by(stmt)?;

        if let Some(cursor) = cursor_object.cursor {
            let values = decode_cursor(&cursor)?;

            let cursor_values: sea_orm::sea_query::value::ValueTuple = map_cursor_values(values)?;

            stmt.after(cursor_values);
        }

        let data = stmt.first(cursor_object.limit).all(db).await?;

        let has_next_page: bool = {
            let mut next_stmt = apply_stmt_cursor_by(next_stmt)?;

            let last_node = data.last();

            if let Some(node) = last_node {
                let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                    .map(|variant| node.get(variant.into_column()))
                    .collect();

                let values = map_cursor_values(values)?;

                let next_data = next_stmt.first(1).after(values).all(db).await?;

                !next_data.is_empty()
            } else {
                false
            }
        };

        let has_previous_page: bool = {
            let mut previous_stmt = apply_stmt_cursor_by(previous_stmt)?;

            let first_node = data.first();

            if let Some(node) = first_node {
                let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                    .map(|variant| node.get(variant.into_column()))
                    .collect();

                let values = map_cursor_values(values)?;

                let previous_data = previous_stmt.first(1).before(values).all(db).await?;

                !previous_data.is_empty()
            } else {
                false
            }
        };

        let edges: Vec<Edge<T>> = data
            .into_iter()
            .map(|node| {
                let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                    .map(|variant| node.get(variant.into_column()))
                    .collect();

                let cursor: String = encode_cursor(values);

                Edge { cursor, node }
            })
            .collect();

        let start_cursor = edges.first().map(|edge| edge.cursor.clone());
        let end_cursor = edges.last().map(|edge| edge.cursor.clone());

        Ok(Connection {
            edges,
            page_info: PageInfo {
                has_previous_page,
                has_next_page,
                start_cursor,
                end_cursor,
            },
            pagination_info: None,
        })
    } else if let Some(page_object) = pagination.page {
        check_limit(context, page_object.limit)?;

        let paginator = stmt.paginate(db, page_object.limit);

        let paginator_info = paginator.num_items_and_pages().await?;

        let data = paginator.fetch_page(page_object.page).await?;

        let edges: Vec<Edge<T>> = data
            .into_iter()
            .map(|node| {
                let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                    .map(|variant| node.get(variant.into_column()))
                    .collect();

                let cursor: String = encode_cursor(values);

                Edge { cursor, node }
            })
            .collect();

        let start_cursor = edges.first().map(|edge| edge.cursor.clone());
        let end_cursor = edges.last().map(|edge| edge.cursor.clone());

        Ok(Connection {
            edges,
            page_info: PageInfo {
                has_previous_page: page_object.page != 0,
                has_next_page: page_object.page + 1 != paginator_info.number_of_pages,
                start_cursor,
                end_cursor,
            },
            pagination_info: Some(PaginationInfo {
                pages: paginator_info.number_of_pages,
                current: page_object.page,
                offset: page_object.page * page_object.limit,
                total: paginator_info.number_of_items,
            }),
        })
    } else if let Some(offset_object) = pagination.offset {
        check_limit(context, offset_object.limit)?;

        let offset = offset_object.offset;
        let limit = offset_object.limit;

        let count_stmt = stmt.clone().as_query().to_owned();

        let data = stmt.offset(offset).limit(limit).all(db).await?;

        let edges: Vec<Edge<T>> = data
            .into_iter()
            .map(|node| {
                let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                    .map(|variant| node.get(variant.into_column()))
                    .collect();

                let cursor: String = encode_cursor(values);

                Edge { cursor, node }
            })
            .collect();

        let start_cursor = edges.first().map(|edge| edge.cursor.clone());
        let end_cursor = edges.last().map(|edge| edge.cursor.clone());

        let count_query = sea_orm::sea_query::SelectStatement::new()
            .expr(sea_orm::sea_query::Expr::cust("COUNT(*) AS num_items"))
            .from_subquery(count_stmt, sea_orm::sea_query::Alias::new("sub_query"))
            .take();

        let total = match db.query_one(&count_query).await? {
            Some(res) => match db.get_database_backend() {
                sea_orm::DbBackend::Postgres => res.try_get::<i64>("", "num_items")? as u64,
                _ => res.try_get::<i32>("", "num_items")? as u64,
            },
            None => 0,
        };

        Ok(Connection {
            edges,
            page_info: PageInfo {
                has_previous_page: offset != 0,
                has_next_page: offset + limit < total,
                start_cursor,
                end_cursor,
            },
            pagination_info: Some(PaginationInfo {
                current: f64::ceil(offset as f64 / limit as f64) as u64,
                pages: f64::ceil(total as f64 / limit as f64) as u64,
                total,
                offset,
            }),
        })
    } else {
        let data = stmt.all(db).await?;

        let edges: Vec<Edge<T>> = data
            .into_iter()
            .map(|node| {
                let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                    .map(|variant| node.get(variant.into_column()))
                    .collect();

                let cursor: String = encode_cursor(values);

                Edge { cursor, node }
            })
            .collect();

        let start_cursor = edges.first().map(|edge| edge.cursor.clone());
        let end_cursor = edges.last().map(|edge| edge.cursor.clone());

        let total = edges.len() as u64;

        Ok(Connection {
            edges,
            page_info: PageInfo {
                has_previous_page: false,
                has_next_page: false,
                start_cursor,
                end_cursor,
            },
            pagination_info: Some(PaginationInfo {
                pages: 1,
                current: 1,
                offset: 0,
                total,
            }),
        })
    }
}

pub fn apply_memory_pagination<T>(
    context: &'static BuilderContext,
    values: Option<Vec<T::Model>>,
    pagination: PaginationInput,
) -> Result<Connection<T>, sea_orm::DbErr>
where
    T: EntityTrait,
    T::Model: Sync,
{
    let pagination = apply_pagination_defaults(context, pagination);

    let edges: Vec<Edge<T>> = match values {
        Some(data) => {
            let edges: Vec<Edge<T>> = data
                .into_iter()
                .map(|node| {
                    let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                        .map(|variant| node.get(variant.into_column()))
                        .collect();

                    let cursor: String = encode_cursor(values);

                    Edge { cursor, node }
                })
                .collect();
            edges
        }
        None => Vec::new(),
    };

    if let Some(cursor_object) = pagination.cursor {
        check_limit(context, cursor_object.limit)?;
        let total: u64 = edges.len() as u64;
        let pages = f64::ceil(total as f64 / cursor_object.limit as f64) as u64;

        let first_cursor = edges.first().map(|edge| edge.cursor.clone());
        let last_cursor = edges.last().map(|edge| edge.cursor.clone());

        let edges: Vec<Edge<T>> = if let Some(cursor) = cursor_object.cursor {
            edges
                .into_iter()
                .filter(|edge: &Edge<T>| edge.cursor.gt(&cursor))
                .collect()
        } else {
            edges
        };

        let current = f64::ceil(total as f64 / edges.len() as f64 * pages as f64) as u64;

        let edges: Vec<Edge<T>> = edges
            .into_iter()
            .take(cursor_object.limit as usize)
            .collect();

        let start_cursor = edges.first().map(|edge| edge.cursor.clone());
        let end_cursor = edges.last().map(|edge| edge.cursor.clone());

        Ok(Connection {
            edges,
            page_info: PageInfo {
                has_previous_page: !first_cursor.eq(&start_cursor),
                has_next_page: !last_cursor.eq(&end_cursor),
                start_cursor,
                end_cursor,
            },
            pagination_info: Some(PaginationInfo {
                pages,
                current,
                offset: current * cursor_object.limit,
                total,
            }),
        })
    } else if let Some(page_object) = pagination.page {
        check_limit(context, page_object.limit)?;
        let total = edges.len() as u64;
        let pages = f64::ceil(total as f64 / page_object.limit as f64) as u64;

        let edges: Vec<Edge<T>> = edges
            .into_iter()
            .skip(
                (page_object.page * page_object.limit)
                    .try_into()
                    .expect("Integer out of range"),
            )
            .take(page_object.limit.try_into().expect("Integer out of range"))
            .collect();

        let start_cursor = edges.first().map(|edge| edge.cursor.clone());
        let end_cursor = edges.last().map(|edge| edge.cursor.clone());

        Ok(Connection {
            edges,
            page_info: PageInfo {
                has_previous_page: page_object.page != 0,
                has_next_page: page_object.page + 1 < pages,
                start_cursor,
                end_cursor,
            },
            pagination_info: Some(PaginationInfo {
                pages,
                current: page_object.page,
                offset: page_object.page * page_object.limit,
                total,
            }),
        })
    } else if let Some(offset_object) = pagination.offset {
        check_limit(context, offset_object.limit)?;
        let total = edges.len() as u64;
        let pages = f64::ceil(total as f64 / offset_object.limit as f64) as u64;
        let current = f64::ceil(offset_object.offset as f64 / offset_object.limit as f64) as u64;

        let edges: Vec<Edge<T>> = edges
            .into_iter()
            .skip(
                offset_object
                    .offset
                    .try_into()
                    .expect("Integer out of range"),
            )
            .take(
                offset_object
                    .limit
                    .try_into()
                    .expect("Integer out of range"),
            )
            .collect();

        let start_cursor = edges.first().map(|edge| edge.cursor.clone());
        let end_cursor = edges.last().map(|edge| edge.cursor.clone());

        Ok(Connection {
            edges,
            page_info: PageInfo {
                has_previous_page: offset_object.offset != 0,
                has_next_page: offset_object.offset + offset_object.limit < total,
                start_cursor,
                end_cursor,
            },
            pagination_info: Some(PaginationInfo {
                pages,
                current,
                offset: offset_object.offset,
                total,
            }),
        })
    } else {
        let start_cursor = edges.first().map(|edge| edge.cursor.clone());
        let end_cursor = edges.last().map(|edge| edge.cursor.clone());

        let total = edges.len() as u64;

        Ok(Connection {
            edges,
            page_info: PageInfo {
                has_previous_page: false,
                has_next_page: false,
                start_cursor,
                end_cursor,
            },
            pagination_info: Some(PaginationInfo {
                pages: 1,
                current: 1,
                offset: 0,
                total,
            }),
        })
    }
}

fn apply_pagination_defaults(
    context: &'static BuilderContext,
    pagination: PaginationInput,
) -> PaginationInput {
    // If there are no pagination options supplied, but a default of or maximum limit has been
    // configured, use page-based pagination with a page number of 0 and the lower of the two
    // applied.

    if pagination.cursor.is_some() || pagination.page.is_some() || pagination.offset.is_some() {
        return pagination;
    }

    let opts = &context.pagination_input;
    let use_limit = match (opts.default_limit, opts.max_limit) {
        (None, None) => None,
        (None, Some(max_limit)) => Some(max_limit),
        (Some(default_limit), None) => Some(default_limit),
        (Some(default_limit), Some(max_limit)) => Some(std::cmp::min(default_limit, max_limit)),
    };

    if let Some(use_limit) = use_limit {
        PaginationInput {
            cursor: None,
            offset: None,
            page: Some(PageInput {
                page: 0,
                limit: use_limit,
            }),
        }
    } else {
        pagination
    }
}

fn check_limit(
    context: &'static BuilderContext,
    requested_limit: u64,
) -> Result<(), sea_orm::DbErr> {
    if requested_limit == 0 {
        return Err(sea_orm::DbErr::Query(sea_orm::RuntimeErr::Internal(
            "Requested pagination limit must be greater than 0".to_string(),
        )));
    }

    if let Some(max_limit) = context.pagination_input.max_limit {
        if requested_limit > max_limit {
            return Err(sea_orm::DbErr::Query(sea_orm::RuntimeErr::Internal(
                format!(
                    "Requested pagination limit ({requested_limit}) exceeds maximum allowed ({max_limit})"
                ),
            )));
        }
    }

    Ok(())
}
