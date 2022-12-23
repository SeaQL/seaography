//! <div align="center">
//!
//!   <img src="https://raw.githubusercontent.com/SeaQL/seaography/main/docs/Seaography.png" width="280" alt="Seaography logo"/>
//!
//!   <p>
//!     <strong>🧭 A GraphQL framework and code generator for SeaORM</strong>
//!   </p>
//!
//!   [![crate](https://img.shields.io/crates/v/seaography.svg)](https://crates.io/crates/seaography)
//!   [![docs](https://docs.rs/seaography/badge.svg)](https://docs.rs/seaography)
//!   [![build status](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml/badge.svg)](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml)
//!
//! </div>
//!
//! # Seaography
//!
//! #### Seaography is a GraphQL framework for building GraphQL resolvers using SeaORM entities. It ships with a CLI tool that can generate ready-to-compile Rust GraphQL servers from existing MySQL, Postgres and SQLite databases.
//!
//! ## Benefits
//!
//! * Quick and easy to get started
//! * Generates readable code
//! * Extensible project structure
//! * Based on popular async libraries: [async-graphql](https://github.com/async-graphql/async-graphql) and [SeaORM](https://github.com/SeaQL/sea-orm)
//!
//! ## Features
//!
//! * Relational query (1-to-1, 1-to-N)
//! * Pagination on query's root entity
//! * Filter with operators (e.g. gt, lt, eq)
//! * Order by any column
//!
//! (Right now there is no mutation, but it's on our plan!)
//!
//! ## Quick start - ready to serve in 3 minutes!
//!
//! ### Install
//!
//! ```sh
//! cargo install seaography-cli
//! ```
//!
//! ### MySQL
//!
//! Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/mysql/sakila-schema.sql) sample database.
//!
//! ```sh
//! cd examples/mysql
//! seaography-cli mysql://user:pw@localhost/sakila seaography-mysql-example .
//! cargo run
//! ```
//!
//! Go to http://localhost:8000/ and try out the following queries:
//!
//! #### Fetch films and their actors
//!
//! ```graphql
//! {
//!   film(pagination: { pages: { limit: 10, page: 0 } }, orderBy: { title: ASC }) {
//!     nodes {
//!       title
//!       description
//!       releaseYear
//!       filmActor {
//!         nodes {
//!           actor {
//!             firstName
//!             lastName
//!           }
//!         }
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! #### Fetch store and its employee
//!
//! ```graphql
//! {
//!   store(filters: { storeId: { eq: 1 } }) {
//!     nodes {
//!       storeId
//!       address {
//!         address
//!         address2
//!       }
//!       staff {
//!         firstName
//!         lastName
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! ### Fetch inactive customers with pagination
//!
//! ```graphql
//! {
//!   customer(
//!     filters: { active: { eq: 0 } }
//!     pagination: { pages: { page: 2, limit: 3 } }
//!   ) {
//!     nodes {
//!       customerId
//!       lastName
//!       email
//!     }
//!     pages
//!     current
//!   }
//! }
//! ```
//!
//! ### Fetch inactive customers with offsets
//!
//! ```graphql
//! {
//!   customer(
//!     filters: { active: { eq: 0 } }
//!     pagination: { offset: { skip: 2, take: 3 } }
//!   ) {
//!     nodes {
//!       customerId
//!       lastName
//!       email
//!     }
//!     skip
//!     take
//!     totalCount
//!   }
//! }
//! ```
//!
//! ### The query above using cursor pagination
//!
//! ```graphql
//! {
//!   customer(
//!     filters: { active: { eq: 0 } }
//!     pagination: { cursor: { limit: 3, cursor: "Int[3]:271" } }
//!   ) {
//!     nodes {
//!       customerId
//!       lastName
//!       email
//!     }
//!     pageInfo {
//!       hasPreviousPage
//!       hasNextPage
//!       endCursor
//!     }
//!   }
//! }
//! ```
//!
//! ### Complex query with filters on relations
//!
//! Find all inactive customers, include their address, and their payments with amount greater than 7 ordered by amount the second result
//!
//! ```graphql
//! {
//!   customer(
//!     filters: { active: { eq: 0 } }
//!     pagination: { cursor: { limit: 3, cursor: "Int[3]:271" } }
//!   ) {
//!     nodes {
//!       customerId
//!       lastName
//!       email
//!       address {
//!         address
//!       }
//!       payment(
//!         filters: { amount: { gt: "7" } }
//!         orderBy: { amount: ASC }
//!         pagination: { pages: { limit: 1, page: 1 } }
//!       ) {
//!         nodes {
//!           paymentId
//!           amount
//!         }
//!         pages
//!         current
//!         pageInfo {
//!           hasPreviousPage
//!           hasNextPage
//!         }
//!       }
//!     }
//!     pageInfo {
//!       hasPreviousPage
//!       hasNextPage
//!       endCursor
//!     }
//!   }
//! }
//! ```
//!
//! ### Postgres
//!
//! Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/postgres/sakila-schema.sql) sample database.
//!
//! ```sh
//! cd examples/postgres
//! seaography-cli postgres://user:pw@localhost/sakila seaography-postgres-example .
//! cargo run
//! ```
//!
//! ### SQLite
//!
//! ```sh
//! cd examples/sqlite
//! seaography-cli sqlite://sakila.db seaography-sqlite-example .
//! cargo run
//! ```
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!
//! Seaography is a community driven project. We welcome you to participate, contribute and together build for Rust's future.

pub use heck;
pub use itertools;
pub use seaography_derive as macros;

pub mod type_filter;
pub use type_filter::{FilterTrait, FilterTypeTrait, TypeFilter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum)]
pub enum OrderByEnum {
    Asc,
    Desc,
}

#[derive(Debug, async_graphql::InputObject)]
pub struct PageInput {
    pub limit: u64,
    pub page: u64,
}

#[derive(Debug, async_graphql::InputObject)]
pub struct OffsetInput {
    pub skip: u64,
    pub take: u64,
}

#[derive(Debug, async_graphql::InputObject)]
pub struct CursorInput {
    pub cursor: Option<String>,
    pub limit: u64,
}

#[derive(async_graphql::OneofObject)]
pub enum Pagination {
    Offset(OffsetInput),
    Pages(PageInput),
    Cursor(CursorInput),
}

#[derive(async_graphql::SimpleObject)]
pub struct ExtraPaginationFields {
    pub pages: Option<u64>,
    pub current: Option<u64>,
    pub skip: Option<u64>,
    pub take: Option<u64>,
    pub total_count: Option<u64>,
}

#[derive(Debug)]
pub enum DecodeMode {
    Type,
    Length,
    ColonSkip,
    Data,
}

pub fn map_cursor_values(values: Vec<sea_orm::Value>) -> sea_orm::sea_query::value::ValueTuple {
    if values.len() == 1 {
        sea_orm::sea_query::value::ValueTuple::One(values[0].clone())
    } else if values.len() == 2 {
        sea_orm::sea_query::value::ValueTuple::Two(values[0].clone(), values[1].clone())
    } else if values.len() == 3 {
        sea_orm::sea_query::value::ValueTuple::Three(
            values[0].clone(),
            values[1].clone(),
            values[2].clone(),
        )
    } else {
        panic!("seaography does not support cursors values with size greater than 3")
    }
}

#[derive(Debug)]
pub struct CursorValues(pub Vec<sea_orm::Value>);

impl async_graphql::types::connection::CursorType for CursorValues {
    type Error = String;

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        let chars = s.chars();

        let mut values: Vec<sea_orm::Value> = vec![];

        let mut type_indicator = String::new();
        let mut length_indicator = String::new();
        let mut data_buffer = String::new();
        let mut length = -1;

        let mut mode: DecodeMode = DecodeMode::Type;
        for char in chars {
            match mode {
                DecodeMode::Type => {
                    if char.eq(&'[') {
                        mode = DecodeMode::Length;
                    } else if char.eq(&',') {
                        // SKIP
                    } else {
                        type_indicator.push(char);
                    }
                }
                DecodeMode::Length => {
                    if char.eq(&']') {
                        mode = DecodeMode::ColonSkip;
                        length = length_indicator.parse::<i64>().unwrap();
                    } else {
                        length_indicator.push(char);
                    }
                }
                DecodeMode::ColonSkip => {
                    // skips ':' char
                    mode = DecodeMode::Data;
                }
                DecodeMode::Data => {
                    if length > 0 {
                        data_buffer.push(char);
                        length -= 1;
                    }

                    if length <= 0 {
                        let value: sea_orm::Value = match type_indicator.as_str() {
                            "TinyInt" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::TinyInt(None)
                                } else {
                                    sea_orm::Value::TinyInt(Some(
                                        data_buffer.parse::<i8>().unwrap(),
                                    ))
                                }
                            }
                            "SmallInt" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::SmallInt(None)
                                } else {
                                    sea_orm::Value::SmallInt(Some(
                                        data_buffer.parse::<i16>().unwrap(),
                                    ))
                                }
                            }
                            "Int" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::Int(None)
                                } else {
                                    sea_orm::Value::Int(Some(data_buffer.parse::<i32>().unwrap()))
                                }
                            }
                            "BigInt" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::BigInt(None)
                                } else {
                                    sea_orm::Value::BigInt(Some(
                                        data_buffer.parse::<i64>().unwrap(),
                                    ))
                                }
                            }
                            "TinyUnsigned" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::TinyUnsigned(None)
                                } else {
                                    sea_orm::Value::TinyUnsigned(Some(
                                        data_buffer.parse::<u8>().unwrap(),
                                    ))
                                }
                            }
                            "SmallUnsigned" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::SmallUnsigned(None)
                                } else {
                                    sea_orm::Value::SmallUnsigned(Some(
                                        data_buffer.parse::<u16>().unwrap(),
                                    ))
                                }
                            }
                            "Unsigned" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::Unsigned(None)
                                } else {
                                    sea_orm::Value::Unsigned(Some(
                                        data_buffer.parse::<u32>().unwrap(),
                                    ))
                                }
                            }
                            "BigUnsigned" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::BigUnsigned(None)
                                } else {
                                    sea_orm::Value::BigUnsigned(Some(
                                        data_buffer.parse::<u64>().unwrap(),
                                    ))
                                }
                            }
                            "String" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::String(None)
                                } else {
                                    sea_orm::Value::String(Some(Box::new(
                                        data_buffer.parse::<String>().unwrap(),
                                    )))
                                }
                            }
                            #[cfg(feature = "with-uuid")]
                            "Uuid" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::Uuid(None)
                                } else {
                                    sea_orm::Value::Uuid(Some(Box::new(
                                        data_buffer.parse::<sea_orm::prelude::Uuid>().unwrap(),
                                    )))
                                }
                            }
                            _ => {
                                // FIXME: missing value types
                                panic!("cannot encode current type")
                            }
                        };

                        values.push(value);

                        type_indicator = String::new();
                        length_indicator = String::new();
                        data_buffer = String::new();
                        length = -1;

                        mode = DecodeMode::Type;
                    }
                }
            }
        }

        Ok(Self(values))
    }

    fn encode_cursor(&self) -> String {
        use itertools::Itertools;

        self.0
            .iter()
            .map(|value| -> String {
                match value {
                    sea_orm::Value::TinyInt(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("TinyInt[{}]:{}", value.len(), value)
                        } else {
                            "TinyInt[-1]:".into()
                        }
                    }
                    sea_orm::Value::SmallInt(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("SmallInt[{}]:{}", value.len(), value)
                        } else {
                            "SmallInt[-1]:".into()
                        }
                    }
                    sea_orm::Value::Int(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("Int[{}]:{}", value.len(), value)
                        } else {
                            "Int[-1]:".into()
                        }
                    }
                    sea_orm::Value::BigInt(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("BigInt[{}]:{}", value.len(), value)
                        } else {
                            "BigInt[-1]:".into()
                        }
                    }
                    sea_orm::Value::TinyUnsigned(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("TinyUnsigned[{}]:{}", value.len(), value)
                        } else {
                            "TinyUnsigned[-1]:".into()
                        }
                    }
                    sea_orm::Value::SmallUnsigned(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("SmallUnsigned[{}]:{}", value.len(), value)
                        } else {
                            "SmallUnsigned[-1]:".into()
                        }
                    }
                    sea_orm::Value::Unsigned(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("Unsigned[{}]:{}", value.len(), value)
                        } else {
                            "Unsigned[-1]:".into()
                        }
                    }
                    sea_orm::Value::BigUnsigned(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("BigUnsigned[{}]:{}", value.len(), value)
                        } else {
                            "BigUnsigned[-1]:".into()
                        }
                    }
                    sea_orm::Value::String(value) => {
                        if let Some(value) = value {
                            let value = value.as_ref();
                            format!("String[{}]:{}", value.len(), value)
                        } else {
                            "String[-1]:".into()
                        }
                    }
                    #[cfg(feature = "with-uuid")]
                    sea_orm::Value::Uuid(value) => {
                        if let Some(value) = value {
                            let value = value.as_ref().to_string();
                            format!("Uuid[{}]:{}", value.len(), value)
                        } else {
                            "Uuid[-1]:".into()
                        }
                    }
                    _ => {
                        // FIXME: missing value types
                        panic!(
                            "cannot
                             current type"
                        )
                    }
                }
            })
            .join(",")
    }
}

#[derive(Debug, Clone)]
pub struct RelationKeyStruct<Entity: EnhancedEntity> {
    pub val: sea_orm::Value,
    pub filter: Option<Entity::Filter>,
    pub order_by: Option<Entity::OrderBy>,
}

impl<Entity: EnhancedEntity> PartialEq for RelationKeyStruct<Entity> {
    fn eq(&self, other: &Self) -> bool {
        // TODO temporary hack to solve the following problem
        // let v1 = TestFK(sea_orm::Value::TinyInt(Some(1)));
        // let v2 = TestFK(sea_orm::Value::Int(Some(1)));
        // println!("Result: {}", v1.eq(&v2));

        fn split_at_nth_char(s: &str, p: char, n: usize) -> Option<(&str, &str)> {
            s.match_indices(p)
                .nth(n)
                .map(|(index, _)| s.split_at(index))
        }

        let a = format!("{:?}", self.val);
        let b = format!("{:?}", other.val);

        let a = split_at_nth_char(a.as_str(), '(', 1).map(|v| v.1);
        let b = split_at_nth_char(b.as_str(), '(', 1).map(|v| v.1);

        a.eq(&b)
    }
}

impl<Entity: EnhancedEntity> Eq for RelationKeyStruct<Entity> {}

impl<Entity: EnhancedEntity> std::hash::Hash for RelationKeyStruct<Entity> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // TODO this is a hack

        fn split_at_nth_char(s: &str, p: char, n: usize) -> Option<(&str, &str)> {
            s.match_indices(p)
                .nth(n)
                .map(|(index, _)| s.split_at(index))
        }

        let a = format!("{:?}", self.val);
        let a = split_at_nth_char(a.as_str(), '(', 1).map(|v| v.1);

        a.hash(state)
        // TODO else do the following
        // match self.0 {
        //     sea_orm::Value::TinyInt(int) => int.unwrap().hash(state),
        //     sea_orm::Value::SmallInt(int) => int.unwrap().hash(state),
        //     sea_orm::Value::Int(int) => int.unwrap().hash(state),
        //     sea_orm::Value::BigInt(int) => int.unwrap().hash(state),
        //     sea_orm::Value::TinyUnsigned(int) => int.unwrap().hash(state),
        //     sea_orm::Value::SmallUnsigned(int) => int.unwrap().hash(state),
        //     sea_orm::Value::Unsigned(int) => int.unwrap().hash(state),
        //     sea_orm::Value::BigUnsigned(int) => int.unwrap().hash(state),
        //     sea_orm::Value::String(str) => str.unwrap().hash(state),
        //     sea_orm::Value::Uuid(uuid) => uuid.unwrap().hash(state),
        //     _ => format!("{:?}", self.0).hash(state)
        // }
    }
}

pub async fn fetch_relation_data<Entity: EnhancedEntity>(
    keys: Vec<RelationKeyStruct<Entity>>,
    relation: sea_orm::RelationDef,
    reverse: bool,
    db: &sea_orm::DatabaseConnection,
) -> std::result::Result<
    Vec<(
        RelationKeyStruct<Entity>,
        <Entity as sea_orm::EntityTrait>::Model,
    )>,
    sea_orm::error::DbErr,
>
where
    Entity: sea_orm::EntityTrait + EnhancedEntity<Entity = Entity>,
    <Entity::Column as std::str::FromStr>::Err: core::fmt::Debug,
{
    use heck::ToSnakeCase;
    use sea_orm::prelude::*;

    let filters = if !keys.is_empty() {
        keys[0].clone().filter
    } else {
        None
    };

    let order_by = if !keys.is_empty() {
        keys[0].clone().order_by
    } else {
        None
    };

    let keys: Vec<sea_orm::Value> = keys.into_iter().map(|key| key.val).collect();

    // TODO support multiple columns
    let to_column = if reverse {
        <Entity::Column as std::str::FromStr>::from_str(
            relation.from_col.to_string().to_snake_case().as_str(),
        )
        .unwrap()
    } else {
        <Entity::Column as std::str::FromStr>::from_str(
            relation.to_col.to_string().to_snake_case().as_str(),
        )
        .unwrap()
    };

    let stmt = <Entity as sea_orm::EntityTrait>::find();

    let filter = sea_orm::Condition::all().add(to_column.is_in(keys));

    let filter = if let Some(filters) = filters {
        filter.add(filters.filter_condition())
    } else {
        filter
    };

    let stmt = sea_orm::QueryFilter::filter(stmt, filter);

    let stmt = if let Some(order_by) = order_by {
        order_by.order_by(stmt)
    } else {
        stmt
    };

    let data = stmt.all(db).await?.into_iter().map(
        |model: <Entity as EntityTrait>::Model| -> (
            RelationKeyStruct<Entity>,
            <Entity as EntityTrait>::Model,
        ) {
            let key = RelationKeyStruct::<Entity> {
                val: model.get(to_column),
                filter: None,
                order_by: None,
            };

            (key, model)
        },
    );

    Ok(data.collect())
}

pub trait EntityFilter {
    fn filter_condition(&self) -> sea_orm::Condition;
}

pub trait EntityOrderBy<Entity>
where
    Entity: sea_orm::EntityTrait,
{
    fn order_by(&self, stmt: sea_orm::Select<Entity>) -> sea_orm::Select<Entity>;
}

pub trait EnhancedEntity {
    type Entity: sea_orm::EntityTrait;

    type Filter: EntityFilter + Clone;

    type OrderBy: EntityOrderBy<Self::Entity> + Clone;
}

pub fn data_to_connection<T>(
    data: Vec<T::Model>,
    has_previous_page: bool,
    has_next_page: bool,
    pages: Option<u64>,
    current: Option<u64>,
    skip: Option<u64>,
    take: Option<u64>,
    total_count: Option<u64>,
) -> async_graphql::types::connection::Connection<
    String,
    T::Model,
    ExtraPaginationFields,
    async_graphql::types::connection::EmptyFields,
>
where
    T: sea_orm::EntityTrait,
    <T as sea_orm::EntityTrait>::Model: async_graphql::OutputType,
{
    use async_graphql::connection::CursorType;
    use sea_orm::{Iterable, ModelTrait, PrimaryKeyToColumn};

    let edges: Vec<
        async_graphql::types::connection::Edge<
            String,
            T::Model,
            async_graphql::types::connection::EmptyFields,
        >,
    > = data
        .into_iter()
        .map(|node| {
            let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                .map(|variant| node.get(variant.into_column()))
                .collect();

            let cursor_string = CursorValues(values).encode_cursor();

            async_graphql::types::connection::Edge::new(cursor_string, node)
        })
        .collect();

    let mut result = async_graphql::types::connection::Connection::<
        String,
        T::Model,
        ExtraPaginationFields,
        async_graphql::types::connection::EmptyFields,
    >::with_additional_fields(
        has_previous_page,
        has_next_page,
        ExtraPaginationFields {
            pages,
            current,
            skip,
            take,
            total_count,
        },
    );

    result.edges.extend(edges);

    result
}
