# PostgreSQL

## Getting started

1. `psql -q postgres://sea:sea@localhost/postgres -c 'CREATE DATABASE "sakila"'`
2. `psql -q postgres://sea:sea@localhost/sakila < sakila-schema.sql`
3. `psql -q postgres://sea:sea@localhost/sakila < sakila-data.sql`
4. `cargo run`

### Troubleshooting

#### Different database username and password
Replace `postgres:postgres` with `username:password` in src/main.rs