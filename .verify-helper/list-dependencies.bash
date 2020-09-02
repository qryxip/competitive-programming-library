#!/bin/bash
LIB_NAME=lib
lib_manifest_dir="$(realpath "$(dirname "$0")/..")/$LIB_NAME"
src_path="$(realpath "$1")"
cd "$(dirname "$src_path")" || exit 1
if [ "$(cargo metadata --format-version 1 --no-deps | jq --arg src_path "$src_path" '.packages[].targets[] | select(.src_path == $src_path) | .kind == ["bin"]')" == true ]; then
  printf '%s' "$(cargo equip --src "$src_path" || exit 1)" | sed -nr 's;^//! - `([a-zA-Z0-9_]+)::([a-zA-Z0-9_]+)` → .*$;\2;p' | xargs -rn 1 printf "%s/src/%s.rs\n" "$lib_manifest_dir"
elif [ "$(basename "$src_path")" = lib.rs ]; then
  false # How do I exclude `lib.rs`?
else
  mod_dependencies="$(cargo metadata --format-version 1 --no-deps | jq '.packages[].metadata."cargo-equip-lib"."mod-dependencies"')" || exit 1
  if [ "$mod_dependencies" = null ]; then
    false # `package.metadata.cargo-equip-lib` is missing
  else
    basename="$(basename "$src_path")"
    mod_dependencies="$(printf '%s' "$mod_dependencies" | jq --arg key "${basename%%.*}" '.[$key]')"
    if [ "$mod_dependencies" = null ]; then
      false # missing key
    else
      printf '%s' "$mod_dependencies" | jq -r '.[]' | xargs -rn 1 printf "%s/%s.rs\n" "$PWD"
    fi
  fi
fi
