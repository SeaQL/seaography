#!/bin/bash
set -e

cd generator
cargo publish
cd ..

cd cli
cargo publish
cd ..

cargo publish