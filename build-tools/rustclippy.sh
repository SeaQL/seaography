#!/bin/bash

set -e

if [ ! -d ./build-tools ]; then

    echo "Please execute this script from the repository root."
    exit 1
    
fi

crates=(`find . -type f -name 'Cargo.toml'`)
for crate in "${crates[@]}"; do
    echo "cargo clippy --manifest-path ${crate} --fix --allow-dirty --allow-staged"
    cargo clippy --manifest-path "${crate}" --fix --allow-dirty --allow-staged
done
