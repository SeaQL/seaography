<div align="center">

  <img src="https://raw.githubusercontent.com/SeaQL/seaography/main/docs/Seaography.png" width="280" alt="Seaography logo"/>

  <p><strong>🧭 A GraphQL framework for Rust</strong></p>
  <p>The quickest way to launch a GraphQL backend</p>

  [![crate](https://img.shields.io/crates/v/seaography.svg)](https://crates.io/crates/seaography)
  [![docs](https://docs.rs/seaography/badge.svg)](https://docs.rs/seaography)
  [![build status](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml/badge.svg)](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml)

</div>

# Seaography

## Introduction

Seaography is a **powerful and extensible GraphQL framework for Rust** that bridges [SeaORM](https://www.sea-ql.org/SeaORM/) and [async-graphql](https://github.com/async-graphql/async-graphql),
turning your database schema into a fully-typed GraphQL API with minimal effort.
By leveraging async-graphql's dynamic schema engine, Seaography avoids the heavy code generation of static approaches, resulting in faster compile times.
The generated schema stays in sync with your SeaORM entities, while still giving you full control to extend and customize it.

With Seaography you can focus on application logic instead of boilerplate. It enables you to:

+ Expose a complete GraphQL schema directly from your SeaORM entities, including filters, pagination, and nested relations
+ Use derive macros to define custom input/output objects, queries, and mutations, and seamlessly mix them with SeaORM models
+ Generate ready-to-run GraphQL servers via the included CLI, supporting different web frameworks out of the box
+ Use RBAC, guards, and lifecycle hooks to implement authorization and custom business logic

## Supported technologies

### Databases

Seaography is built on top of SeaORM, so it supports:

+ MySQL, PostgreSQL and SQLite
+ SQL Server (via [SeaORM X](https://www.sea-ql.org/SeaORM-X/))

### Web framework

It's easy to integrate Seaography with any web framework, and we ship with the following examples out-of-the-box:

+ [Actix](https://github.com/SeaQL/seaography/tree/1.1.x/examples/mysql), [Axum](https://github.com/SeaQL/seaography/tree/1.1.x/examples/postgres), [Poem](https://github.com/SeaQL/seaography/tree/1.1.x/examples/sqlite)
+ [Loco (SeaORM)](https://github.com/SeaQL/sea-orm/tree/master/examples/loco_seaography), [Loco (SeaORM Pro)](https://github.com/SeaQL/sea-orm-pro)

### SeaORM Version Compatibility

|                        Seaography                        |                         SeaORM                        |
|----------------------------------------------------------|-------------------------------------------------------|
| [2.0](https://crates.io/crates/seaography/2.0.0-rc)      | [2.0](https://crates.io/crates/sea-orm/2.0.0-rc)      |
| [1.1](https://crates.io/crates/seaography/1.1.4)         | [1.1](https://crates.io/crates/sea-orm/1.1.13)        |

## Features

* Rich types support (e.g. DateTime, Decimal)
* Relational query (1-to-1, 1-to-N, M-to-N)
* Offset-based and cursor-based pagination
* Filtering with operators (e.g. gt, lt, eq)
* Filter by related entities
* Order by any column
* Mutations (create, update, delete)
* Guards and Filters on entity to restrict access
* Choose between camel or snake case field names

### Extensible

Seaography is also completely extensible. It offers:

* Extensive configuration options in schema builder
* Lifecycle hooks for custom resolver logic
* Add custom queries & mutations with derive macros

## Quick start - ready to serve in 3 minutes!

### Install

```sh
cargo install sea-orm-cli@^2.0.0-rc # used to generate entities
cargo install seaography-cli@^2.0.0-rc
```

### MySQL

Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/mysql/sakila-schema.sql) sample database.
Then regenerate example project like below, or simply do `cargo run`.

```sh
cd examples/mysql
sea-orm-cli generate entity -o src/entities -u mysql://user:pw@127.0.0.1/sakila --seaography
seaography-cli -o ./ -e src/entities -u mysql://user:pw@127.0.0.1/sakila seaography-mysql-example
cargo run
```

### Postgres

Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/postgres/sakila-schema.sql) sample database.
Then regenerate example project like below, or simply do `cargo run`.

```sh
cd examples/postgres
sea-orm-cli generate entity -o src/entities -u postgres://user:pw@localhost/sakila --seaography
seaography-cli -o ./ -e src/entities -u postgres://user:pw@localhost/sakila seaography-postgres-example
cargo run
```

### SQLite

`sakila.db` is shipped with this repository. You don't have to setup anything, simply do `cargo run`.

```sh
cd examples/sqlite
sea-orm-cli generate entity -o src/entities -u sqlite://sakila.db --seaography
seaography-cli -o ./ -e src/entities -u sqlite://sakila.db seaography-sqlite-example
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

### Filter using MySQL / Postgres enum
```graphql
{
  film(
    filters: { rating: { eq: NC17 } }
    pagination: { page: { page: 1, limit: 5 } }
  ) {
    nodes {
      filmId
      title
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
