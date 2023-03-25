#!/bin/bash
set -e

cd generator
cargo publish
cd ..
sleep 10

cd cli
cargo publish
cd ..
sleep 10

cargo publish