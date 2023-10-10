<div align="center">

  <img src="https://raw.githubusercontent.com/SeaQL/seaography/main/docs/Seaography.png" width="280" alt="Seaography logo"/>

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
* Pagination for queries and relations (1-N)
* Filtering with operators (e.g. gt, lt, eq)
* Order by any column
* Guard fields, queries or relations
* Rename fields
* Mutations (create, update, delete)

(Right now there is no mutation, but it's on our plan!)

## Quick start - ready to serve in 3 minutes!

### Install

```sh
cargo install sea-orm-cli@^0.12 # used to generate entities
cargo install seaography-cli@^1.0.0-rc.2
```

### MySQL

Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/mysql/sakila-schema.sql) sample database.

```sh
cd examples/mysql
sea-orm-cli generate entity -o src/entities -u mysql://user:pw@127.0.0.1/sakila --seaography
seaography-cli ./ src/entities mysql://user:pw@127.0.0.1/sakila seaography-mysql-example
cargo run
```

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

### Postgres

Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/postgres/sakila-schema.sql) sample database.

```sh
cd examples/postgres
sea-orm-cli generate entity -o src/entities -u postgres://user:pw@localhost/sakila --seaography
seaography-cli ./ src/entities postgres://user:pw@localhost/sakila seaography-postgres-example
cargo run
```

### SQLite

```sh
cd examples/sqlite
sea-orm-cli generate entity -o src/entities -u sqlite://sakila.db --seaography
seaography-cli ./ src/entities sqlite://sakila.db seaography-sqlite-example
cargo run
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

Seaography is a community driven project. We welcome you to participate, contribute and together build for Rust's future.
