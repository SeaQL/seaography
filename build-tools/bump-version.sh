#!/bin/bash
set -e

# Bump `seaography-generator` version
cd generator
sed -i 's/^version.*$/version = "'$1'"/' Cargo.toml
cd ..

# Bump `seaography-cli` version
cd cli
sed -i 's/^version.*$/version = "'$1'"/' Cargo.toml
sed -i 's/^seaography-generator [^,]*,/seaography-generator = { version = "~'$1'",/' Cargo.toml
cd ..

# Bump `seaography` version
sed -i 's/^version.*$/version = "'$1'"/' Cargo.toml
git commit -am "$1"

# Bump examples' dependency version
cd examples
find . -depth -type f -name '*.toml' -exec sed -i 's/^version = ".*" # seaography version$/version = "~'$1'" # seaography version/' {} \;
find . -depth -type f -name '*.toml' -exec sed -i 's/^version = "[0-9].*"$/version = "'$1'"/' {} \;
git commit -am "update examples"
