<div align="center">

  <h1>
    <img src="https://raw.githubusercontent.com/SeaQL/seaography/main/docs/Seaography.png" width="280 alt="Seaography"/>
  </h1>

  <p>
    <strong>🧭 A GraphQL framework and code generator for SeaORM</strong>
  </p>

  [![crate](https://img.shields.io/crates/v/seaography.svg)](https://crates.io/crates/seaography)
  [![docs](https://docs.rs/seaography/badge.svg)](https://docs.rs/seaography)
  [![build status](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml/badge.svg)](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml)

</div>

# Seaography

#### Seaography is a GraphQL framework for building GraphQL resolvers using SeaORM entities. It ships with a CLI tool that can generate ready-to-compile Rust GraphQL servers from existing MySQL, Postgres and SQLite databases.

## Benefits

* Quick and easy to get started
* Generates readable code
* Extensible project structure
* Based on popular async libraries: [async-graphql](https://github.com/async-graphql/async-graphql) and [SeaORM](https://github.com/SeaQL/sea-orm)

## Features

* Relational query (1-to-1, 1-to-N)
* Pagination on query's root entity
* Filter with operators (e.g. gt, lt, eq)
* Order by any column

(Right now there is no mutation, but it's on our plan!)

## Quick start - ready to serve in 3 minutes!

### Install

```sh
cargo install seaography-cli
```

### MySQL

Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/mysql/sakila-schema.sql) sample database.

```sh
cd examples/mysql
seaography-cli mysql://user:pw@localhost/sakila seaography-mysql-example .
cargo run
```

Go to http://localhost:8000/ and try out the following queries:

#### Fetch films and their actors

```graphql
{
  film(pagination: { pages: { limit: 10, page: 0 } }, orderBy: { title: ASC }) {
    nodes {
      title
      description
      releaseYear
      filmActor {
        nodes {
          actor {
            firstName
            lastName
          }
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
    pagination: { pages: { page: 2, limit: 3 } }
  ) {
    nodes {
      customerId
      lastName
      email
    }
    pages
    current
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
        pagination: { pages: { limit: 1, page: 1 } }
      ) {
        nodes {
          paymentId
          amount
        }
        pages
        current
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

### Postgres

Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/postgres/sakila-schema.sql) sample database.

```sh
cd examples/postgres
seaography-cli postgres://user:pw@localhost/sakila seaography-postgres-example .
cargo run
```

### SQLite

```sh
cd examples/sqlite
seaography-cli sqlite://sakila.db seaography-sqlite-example .
cargo run
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

Seaography is a community driven project. We welcome you to participate, contribute and together build for Rust's future.
