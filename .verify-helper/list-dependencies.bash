#!/bin/bash
src_path="$(realpath "$1")"
cd "$(dirname "$src_path")" || exit 1
if [ "$(cargo metadata --format-version 1 --no-deps | jq --arg src_path "$src_path" '.packages[].targets[] | select(.src_path == $src_path) | .kind == ["bin"]')" == true ]; then
  bundled="$(cargo equip --check --src "$src_path")" || exit 1
  bin_pkg_name="$(cargo metadata --format-version 1 --no-deps | jq -r --arg src_path "$src_path" '.packages[] | select(.targets | any(.src_path == $src_path)) | .name')" || exit 1
  lib_manifest_path="$(cargo metadata --format-version 1 | jq -r --arg bin_pkg_name "$bin_pkg_name" '.packages[] | select(.source == null and .name != $bin_pkg_name) | .manifest_path')" || exit 1
  printf '%s' "$bundled" | sed -nr 's;^//! - `([a-zA-Z0-9_]+)::([a-zA-Z0-9_]+)` → .*$;\2;p' | xargs -rn 1 printf "%s/src/%s.rs\n" "$(dirname "$lib_manifest_path")"
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
