<div align="center">

  <img src="https://raw.githubusercontent.com/SeaQL/seaography/main/docs/SeaQL logo.png" width="280"/>

  <h1>Seaography</h1>

  <p>
    <strong>🧭 A GraphQL framework and code generator for SeaORM</strong>
  </p>

  [![crate](https://img.shields.io/crates/v/seaography.svg)](https://crates.io/crates/seaography)
  [![docs](https://docs.rs/seaography/badge.svg)](https://docs.rs/seaography)
  [![build status](https://github.com/SeaQL/seaography/actions/workflows/tests.yml/badge.svg)](https://github.com/SeaQL/seaography/actions/workflows/tests.yml)

</div>

# Seaography

#### Seaography is a GraphQL framework for building GraphQL resolvers using SeaORM entities. It ships with a CLI tool that can generate ready-to-compile Rust GraphQL servers from existing MySQL, Postgres and SQLite databases.

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

Go to http://localhost:8000/ and try out the following query:

```graphql
{
  film(pagination: { limit: 10, page: 0 }, orderBy: { title: ASC }) {
    data {
      title
      description
      releaseYear
      filmActor {
        actor {
          firstName
          lastName
        }
      }
    }
  }
}
```

### Postgres

Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/postgres/sakila-schema.sql) sample database.

```sh
cd examples/postgres
seaography-cli postgres://user:pw@localhost/sakila seaography-example-postgres .
cargo run
```

### SQLite

```sh
cd examples/sqlite
seaography-cli sqlite://chinook.db seaography-sqlite-example .
cargo run
```

Go to http://localhost:8000/ and try out the following query:

```graphql
{
  albums(pagination: { limit: 10, page: 0 }) {
    data {
      title
      artists {
        name
      }
    }
  }
}
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

Seaography is a community driven project. We welcome you to participate, contribute and together build for Rust's future.