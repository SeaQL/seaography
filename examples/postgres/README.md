# PostgreSQL

## Getting started

```sh
psql -q postgres://sea:sea@localhost/postgres -c 'CREATE DATABASE "sakila"'
psql -q postgres://sea:sea@localhost/sakila < sakila-schema.sql
psql -q postgres://sea:sea@localhost/sakila < sakila-data.sql
psql -q postgres://sea:sea@localhost/sakila < sakila-patch.sql
```

```sh
cargo run
```

### Troubleshooting

#### Different database username and password
Replace `sea:sea` with `username:password` in `.env`