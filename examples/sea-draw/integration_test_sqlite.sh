#!/bin/bash
set -eu
rm -f sea_draw.db
sqlite3 sea_draw.db < schema/sqlite.sql
export DATABASE_URL="sqlite://sea_draw.db"
cargo run --bin integration_test
