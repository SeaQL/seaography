#!/bin/bash
set -e

cd derive
cargo publish
cd ..
sleep 10

cd discoverer
cargo publish
cd ..
sleep 10

cd generator
cargo publish
cd ..
sleep 10

cargo publish