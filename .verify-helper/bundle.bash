#!/bin/bash
src_path="$(realpath "$1")"
cd "$(dirname "$src_path")" || exit 1
if [ "$(cargo metadata --format-version 1 --no-deps | jq --arg src_path "$src_path" '.packages[].targets[] | select(.src_path == $src_path) | .kind == ["bin"]')" == true ]; then
  cargo equip --oneline mods --rustfmt
else
  basename="$(basename "$src_path")"
  printf "mod %s{%s}\n" "${basename%%.*}" "$(cat "$src_path")"
fi
