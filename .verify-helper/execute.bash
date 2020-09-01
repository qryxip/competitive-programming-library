#!/bin/bash
cd "$(dirname "$1")" || exit 1
basename="$(basename "$1")"
eval "$(cargo metadata --format-version 1 --no-deps | jq -r .target_directory || exit 1)/release/${basename%%.*}"
