#!/bin/sh

set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <cargo-bin-name>" >&2
    exit 2
fi

binary_name=$1
database_url=postgres://rust_training:rust_training@localhost:5433/rust_training

cleanup() {
    docker compose down --volumes --remove-orphans
}

trap cleanup EXIT INT TERM

docker compose up -d --wait postgres
DATABASE_URL="$database_url" cargo test --bin "$binary_name" -- --include-ignored
