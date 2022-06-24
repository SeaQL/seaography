#! /bin/bash
cd seaography_discoverer
cargo run -- --url="sqlite:../chinook.db" > ../schema.json