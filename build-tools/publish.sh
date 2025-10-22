#!/bin/bash
set -e

cd generator
cargo publish
cd ..

cd cli
cargo publish
cd ..

cd macros
cargo publish
cd ..

cargo publish