## Database

The data come from the following sample database https://www.postgresqltutorial.com/postgresql-getting-started/postgresql-sample-database/

## Getting started

1. `psql -q postgres://sea:sea@localhost/postgres -c 'CREATE DATABASE "dvdrental"'`
2. `psql -q postgres://sea:sea@localhost/dvdrental < dvdrental-schema.sql`
3. `psql -q postgres://sea:sea@localhost/dvdrental < dvdrental-data.sql`
4. `cargo run`

### Troubleshooting

#### Different database username and password
Replace `sea:sea` with `username:password` on current folder.