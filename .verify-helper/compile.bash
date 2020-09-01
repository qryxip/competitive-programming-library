#!/bin/bash
cd "$(dirname "$1")" || exit 1
cargo build --release
