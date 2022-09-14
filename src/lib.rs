//! <div align="center">
//!
//!   <h1>
//!     <img src="https://raw.githubusercontent.com/SeaQL/seaography/main/docs/Seaography.png" width="280 alt="Seaography"/>
//!   </h1>
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
//!   film(pagination: { limit: 10, page: 0 }, orderBy: { title: ASC }) {
//!     data {
//!       title
//!       description
//!       releaseYear
//!       filmActor {
//!         actor {
//!           firstName
//!           lastName
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
//!     data {
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
//! seaography-cli sqlite://chinook.db seaography-sqlite-example .
//! cargo run
//! ```
//!
//! Go to http://localhost:8000/ and try out the following query:
//!
//! #### Fetch albums and their artists
//!
//! ```graphql
//! {
//!   albums(pagination: { limit: 10, page: 0 }) {
//!     data {
//!       title
//!       artists {
//!         name
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!
//! Seaography is a community driven project. We welcome you to participate, contribute and together build for Rust's future.

pub use seaography_derive as macros;

#[derive(Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum)]
pub enum OrderByEnum {
    Asc,
    Desc,
}

pub type BinaryVector = Vec<u8>;

#[derive(Debug, async_graphql::InputObject)]
#[graphql(concrete(name = "StringFilter", params(String)))]
#[graphql(concrete(name = "TinyIntegerFilter", params(i8)))]
#[graphql(concrete(name = "SmallIntegerFilter", params(i16)))]
#[graphql(concrete(name = "IntegerFilter", params(i32)))]
#[graphql(concrete(name = "BigIntegerFilter", params(i64)))]
#[graphql(concrete(name = "TinyUnsignedFilter", params(u8)))]
#[graphql(concrete(name = "SmallUnsignedFilter", params(u16)))]
#[graphql(concrete(name = "UnsignedFilter", params(u32)))]
#[graphql(concrete(name = "BigUnsignedFilter", params(u64)))]
#[graphql(concrete(name = "FloatFilter", params(f32)))]
#[graphql(concrete(name = "DoubleFilter", params(f64)))]
// TODO #[graphql(concrete(name = "JsonFilter", params()))]
// TODO #[graphql(concrete(name = "DateFilter", params()))]
// TODO #[graphql(concrete(name = "TimeFilter", params()))]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateFilter", params(sea_orm::prelude::Date)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateTimeFilter", params(sea_orm::prelude::DateTime)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateTimeUtcFilter", params(sea_orm::prelude::DateTimeUtc)))
)]
// TODO #[graphql(concrete(name = "TimestampFilter", params()))]
// TODO #[graphql(concrete(name = "TimestampWithTimeZoneFilter", params()))]
#[cfg_attr(
    feature = "with-decimal",
    graphql(concrete(name = "DecimalFilter", params(sea_orm::prelude::Decimal)))
)]
#[cfg_attr(
    feature = "with-uuid",
    graphql(concrete(name = "UuidFilter", params(sea_orm::prelude::Uuid)))
)]
#[graphql(concrete(name = "BinaryFilter", params(BinaryVector)))]
#[graphql(concrete(name = "BooleanFilter", params(bool)))]
pub struct TypeFilter<T: async_graphql::InputType> {
    pub eq: Option<T>,
    pub ne: Option<T>,
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
    pub is_in: Option<Vec<T>>,
    pub is_not_in: Option<Vec<T>>,
    pub is_null: Option<bool>,
}
