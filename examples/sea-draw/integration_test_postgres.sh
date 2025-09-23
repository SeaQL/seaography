#!/bin/bash
set -eu
rm -f generic.db
export TRACE=info
export DATABASE_URL="postgres://sea:sea@127.0.0.1/sea_draw_example"

psql -q postgres://sea:sea@localhost/postgres -c 'DROP DATABASE IF EXISTS "sea_draw_example"'
psql -q postgres://sea:sea@localhost/postgres -c 'CREATE DATABASE "sea_draw_example"'

psql -q "$DATABASE_URL" --set=ON_ERROR_STOP=1 sea_draw_example < schema/postgres.sql
#psql -q "$DATABASE_URL" sea_draw_example < schema/postgres.sql

cargo run --bin integration_test
