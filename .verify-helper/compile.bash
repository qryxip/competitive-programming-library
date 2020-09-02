#!/bin/bash
src_path="$(realpath "$1")" || exit 1
cd "$(dirname "$src_path")" || exit 1
bin_name="$(cargo metadata --format-version 1 --no-deps | jq -r --arg src_path "$src_path" '.packages[].targets[] | select(.src_path == $src_path) | .name')" || exit 1
cargo build --release --bin "$bin_name"
