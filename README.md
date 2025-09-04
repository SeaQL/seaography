<div align="center">

  <img src="https://raw.githubusercontent.com/SeaQL/seaography/main/docs/Seaography.png" width="280" alt="Seaography logo"/>

  <p>
    <strong>🧭 A dynamic GraphQL framework for SeaORM</strong>
  </p>

  [![crate](https://img.shields.io/crates/v/seaography.svg)](https://crates.io/crates/seaography)
  [![docs](https://docs.rs/seaography/badge.svg)](https://docs.rs/seaography)
  [![build status](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml/badge.svg)](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml)

</div>

# Seaography

Seaography is a GraphQL framework that bridges async-graphql and SeaORM, instantly turning your database into a fully functional GraphQL API in Rust.
It leverages async‑graphql's dynamic schema capabilities, resulting in minimal generated code and faster compile times compared to static schemas.
With extensive configuration options, you can easily tailor the generated GraphQL schema to your application's needs.

Seaography enables you to focus on your application logic instead of boilerplate.
With Seaography, you can:

+ Turn a set of SeaORM entities into a complete GraphQL schema
+ Use derive macros to craft custom input / output objects, queries and mutations, mix-and-match them with SeaORM models
+ Generate web servers with the included CLI - ready to compile and run

## Supported technologies

### Databases

Seaography is built on top of SeaORM, so it supports:

+ MySQL, PostgreSQL and SQLite
+ SQL Server (via [SeaORM X](https://www.sea-ql.org/SeaORM-X/))

### Web framework

It's easy to integrate Seaography with any web framework, but we ship with the following examples out-of-the-box:

+ [Actix](https://github.com/SeaQL/seaography/tree/1.1.x/examples/sqlite), [Axum](https://github.com/SeaQL/seaography/tree/1.1.x/examples/mysql), [Poem](https://github.com/SeaQL/seaography/tree/1.1.x/examples/postgres)
+ [Loco (SeaORM Pro)](https://github.com/SeaQL/sea-orm-pro)

## Features

* Rich types support (e.g. DateTime, Decimal)
* Relational query (1-to-1, 1-to-N, M-to-N)
* Pagination for queries and relations
* Filtering with operators (e.g. gt, lt, eq)
* Order by any column
* Mutations (create, update, delete)
* Field guards on entity / column to restrict access
* Choose between camel or snake case, and singular or plural field names

## SeaORM Version Compatibility

|                        Seaography                        |                         SeaORM                        |
|----------------------------------------------------------|-------------------------------------------------------|
| [2.0](https://crates.io/crates/seaography/2.0.0-rc)      | [2.0](https://crates.io/crates/sea-orm/2.0.0-rc)      |
| [1.1](https://crates.io/crates/seaography/1.1.4)         | [1.1](https://crates.io/crates/sea-orm/1.1.13)        |

## Quick start - ready to serve in 3 minutes!

### Install

```sh
cargo install sea-orm-cli@^1.1 # used to generate entities
cargo install seaography-cli@^1.1
```

### MySQL

Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/mysql/sakila-schema.sql) sample database.
Then regenerate example project like below, or simply do `cargo run`.

```sh
cd examples/mysql
sea-orm-cli generate entity -o src/entities -u mysql://user:pw@127.0.0.1/sakila --seaography
seaography-cli ./ src/entities mysql://user:pw@127.0.0.1/sakila seaography-mysql-example
cargo run
```

### Postgres

Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/postgres/sakila-schema.sql) sample database.
Then regenerate example project like below, or simply do `cargo run`.

```sh
cd examples/postgres
sea-orm-cli generate entity -o src/entities -u postgres://user:pw@localhost/sakila --seaography
seaography-cli ./ src/entities postgres://user:pw@localhost/sakila seaography-postgres-example
cargo run
```

### SQLite

`sakila.db` is shipped with this repository. You don't have to setup anything, simply do `cargo run`.

```sh
cd examples/sqlite
sea-orm-cli generate entity -o src/entities -u sqlite://sakila.db --seaography
seaography-cli ./ src/entities sqlite://sakila.db seaography-sqlite-example
cargo run
```

## Quick Demo

Go to http://localhost:8000/ and try out the following queries:

#### Fetch films and their actors

```graphql
{
  film(pagination: { page: { limit: 10, page: 0 } }, orderBy: { title: ASC }) {
    nodes {
      title
      description
      releaseYear
      actor {
        nodes {
          firstName
          lastName
        }
      }
    }
  }
}
```

#### Fetch store and its employee

```graphql
{
  store(filters: { storeId: { eq: 1 } }) {
    nodes {
      storeId
      address {
        address
        address2
      }
      staff {
        firstName
        lastName
      }
    }
  }
}
```

### Fetch inactive customers with pagination

```graphql
{
  customer(
    filters: { active: { eq: 0 } }
    pagination: { page: { page: 2, limit: 3 } }
  ) {
    nodes {
      customerId
      lastName
      email
    }
    paginationInfo {
      pages
      current
    }
  }
}
```

### The query above using cursor pagination

```graphql
{
  customer(
    filters: { active: { eq: 0 } }
    pagination: { cursor: { limit: 3, cursor: "Int[3]:271" } }
  ) {
    nodes {
      customerId
      lastName
      email
    }
    pageInfo {
      hasPreviousPage
      hasNextPage
      endCursor
    }
  }
}
```

### Complex query with filters on relations

Find all inactive customers, include their address, and their payments with amount greater than 7 ordered by amount the second result

```graphql
{
  customer(
    filters: { active: { eq: 0 } }
    pagination: { cursor: { limit: 3, cursor: "Int[3]:271" } }
  ) {
    nodes {
      customerId
      lastName
      email
      address {
        address
      }
      payment(
        filters: { amount: { gt: "7" } }
        orderBy: { amount: ASC }
        pagination: { page: { limit: 1, page: 1 } }
      ) {
        nodes {
          paymentId
          amount
        }
        paginationInfo {
          pages
          current
        }
        pageInfo {
          hasPreviousPage
          hasNextPage
        }
      }
    }
    pageInfo {
      hasPreviousPage
      hasNextPage
      endCursor
    }
  }
}
```

### Filter using enumeration
```graphql
{
  film(
    filters: { rating: { eq: NC17 } }
    pagination: { page: { page: 1, limit: 5 } }
  ) {
    nodes {
      filmId
      rating
    }
  }
}
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

We invite you to participate, contribute and together help build Rust's future.

## Mascot

A friend of Ferris, Terres the hermit crab is the official mascot of SeaORM. His hobby is collecting shells.

<img alt="Terres" src="https://www.sea-ql.org/SeaORM/img/Terres.png" width="400"/>
