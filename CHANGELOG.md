# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 0.2.0 - 2022-10-31

## What's Changed
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

## New Contributors
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