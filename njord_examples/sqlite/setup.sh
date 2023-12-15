#!/bin/bash
# -----------------------------------------------------------------------------
# Script: setup.sh
# Description: Build and run the njord_cli application with SQLite support.
# Author: Marcus Cvjeticanin
# Date: December 15, 2023
# -----------------------------------------------------------------------------

cargo build --manifest-path=../../njord_cli/Cargo.toml --no-default-features --features "sqlite"
cargo run --manifest-path=../../njord_cli/Cargo.toml -- setup
