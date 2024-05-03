# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 1.0.0 - Pending

### Versions

+ `1.0.0-rc.3`: 2024-05-02
+ `1.0.0-rc.4`: 2024-05-03

### New Features

* [seaography-cli] option to generate Axum web framework

### Upgrades

* Upgrade `sea-orm` to 1.0.0-rc.3
* Upgrade `async-graphql` to 7.0
* Upgrade `poem` to 3.0

### House keeping

* Drop the use of `async-trait`

## 0.12.0 - 2024-04-29

Introduction the functional API of Seaography. Warning, this version has breaking changes, but it was a sacrifice in order to make the project easier to maintain. With this version we have support for field guards and field renames.

### New Features

* Functional API
* Field renames
* Field guards

* add `update_mutation`

  This module enables the update mutation for entities. The update mutation takes an entity data object with a filter condition object,
  applies the update to the database and returns the modified entities.


* add `delete_mutation`

  This module enables the delete mutation for entities. The delete mutation takes an entity condition filter object,
  deletes the selected entities from database and returns the number of deleted items.

* add `create_one_mutation`

  This module is responsible to allow the Create One mutation. The mutation takes data for a single entity and returns the created entity

* add `create_batch_mutation`

  This module is responsible to allow the Create Batch mutation. The mutation takes and array of data for multiple entities and returns the created entities

* add `entity_input`

  This module is responsible to create a GraphQL input object for an SeaORM entity. Used in the create mutation

### Breaking changes

* Dropped the derive API in favor of a functional API

  SeaORM is a dynamic ORM for rust, this means that we can inspect the Tables, Columns properties on runtime. Recently async-graphql added support for dynamic creation of GraphQL nodes. Utilizing the dynamic nature of both libraries the Derive API is no longer needed and we developed a functional approach API. Moreover, the project in order to live long it needs to be maintainable (easy to maintain) and extensible (easy to extend), but the Derive API was fairly complex compared to a functional API. In order to make the migration easier we updated the seaography generator to generate using the new API

* Decoupled sea-orm-cli from seaography-cli

  Because we don't have to extend the result produced by the sea-orm-cli we decoupled the dependency away fro, seaography in order to make future versions easier to maintain.

* Dataloader optimizations are not introduced yet

  The Dataloader optimizations are going to be added in future versions

* Some renames in Connection node fields, and pagination filtering

### Enhancements

* refactor entity types

  * add `types_map` This is responsible to allow the API user to provide custom entity field types, parsers and formatters
  * refactor `entity_object` to remove code responsible for type mapping

* simplify schema builder

  * register functions
  * internal context

* extend schema builder entity register function to include mutations

* refactor `filtering` functionality

  * add `FilterConfig` for basic types
  * add `filter_types_map` That is responsible to allow the API user to provide custom entity field filter types and condition functions
  * add `apply_condition` that takes `FilterInputConfig` and `condition` as input and updates the `condition`
  * refactor `active_enum_filter_input` to utilize `FilterConfig` and provide `apply_condition` function
  * remove condition code from `filter_input` and utilize `apply_condition` function that uses `FilterConfig`

* start error handling

* slim down code generation for the `query_root.rs` file of a generated project

* update crates

* update examples

## 0.3.0 - 2022-12-02

### New Features

* Relations derive produces the reverse of a self reference relation https://github.com/SeaQL/seaography/pull/99
* Filtering, sorting and paginating related 1-to-many queries (Note: Pagination is WIP, currently in memory only pagination) https://github.com/SeaQL/seaography/pull/84
* Add Actix web framework generator https://github.com/SeaQL/seaography/pull/74
* [seaography-cli] option to generate Actix or Poem web framework https://github.com/SeaQL/seaography/pull/74

### Enhancements

* Generic type filter https://github.com/SeaQL/seaography/pull/97

### Upgrades

* Upgrade `sea-orm` to 0.10 https://github.com/SeaQL/seaography/pull/93

## 0.2.0 - 2022-10-31

### What's Changed
* Conditionally add filter types list with features by @XiNiHa in https://github.com/SeaQL/seaography/pull/67
* Re-export internal dependency instead of leaking it into user package by @billy1624 in https://github.com/SeaQL/seaography/pull/68
* Add --with-json feature by @nicompte in https://github.com/SeaQL/seaography/pull/70
* Refactoring the main function by @billy1624 in https://github.com/SeaQL/seaography/pull/65
* Remove filters print by @nicompte in https://github.com/SeaQL/seaography/pull/71
* Add cursor pagination by @karatakis in https://github.com/SeaQL/seaography/pull/69
* Move root_query basic dependencies into seaography crate by @karatakis in https://github.com/SeaQL/seaography/pull/82
* Add DateTimeWithTimeZone to filter generation by @karatakis in https://github.com/SeaQL/seaography/pull/80
* Allow QueryRoot derive to drive async gql config by @karatakis in https://github.com/SeaQL/seaography/pull/81
* Add ignore_tables and hidden_tables arguments by @karatakis in https://github.com/SeaQL/seaography/pull/79
* Move RelationKey struct to lib by @karatakis in https://github.com/SeaQL/seaography/pull/85

### New Contributors
* @XiNiHa made their first contribution in https://github.com/SeaQL/seaography/pull/67
* @nicompte made their first contribution in https://github.com/SeaQL/seaography/pull/70

**Full Changelog**: https://github.com/SeaQL/seaography/compare/0.1.2...0.2.0

## 0.1.2 - 2022-09-17

* Updated sea-schema to 0.9.4 #62

## 0.1.1 - 2022-09-12

* Replace HashMap with BTreeMap #53
* Decouple discoverer from generator #54

## 0.1.0 - 2022-09-12

* Initial release
