#!/bin/bash
lib_manifest_dir="$(realpath .)/lib"
src_path="$(realpath "$1")"
cd "$(dirname "$src_path")" || exit 1
if [ "$(cargo metadata --format-version 1 --no-deps | jq --arg src_path "$src_path" '.packages[].targets[] | select(.src_path == $src_path) | .kind == ["bin"]')" == true ]; then
  bundled="$(cargo equip --oneline mods --rustfmt)" || exit 1
  printf '%s' "$bundled" | sed -nr 's/^.* mod ([a-z0-9_]+) \{.*$/\1/p' | xargs -rn 1 printf "$lib_manifest_dir/src/%s.rs\n"
else
  mod_dependencies="$(cargo metadata --format-version 1 --no-deps | jq '.packages[].metadata."cargo-equip-lib"."mod-dependencies"')" || exit 1
  if [ "$mod_dependencies" = null ]; then
    false # `package.metadata.cargo-equip-lib` is missing, or `{path}` is a verification file
  else
    basename="$(basename "$src_path")"
    mod_dependencies="$(printf '%s' "$mod_dependencies" | jq --arg key "${basename%%.*}" '.[$key]')"
    if [ "$mod_dependencies" = null ]; then
      false # missing key
    else
      printf '%s' "$mod_dependencies" | jq -r '.[]' | xargs -rn 1 printf "$PWD/%s.rs\n"
    fi
  fi
fi
