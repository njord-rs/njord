#!/bin/bash
# -----------------------------------------------------------------------------
# Script: generate.sh
# Description: Generate new migration files application with SQLite support.
# Author: Marcus Cvjeticanin
# Date: December 15, 2023
# -----------------------------------------------------------------------------
cargo run --manifest-path=../../njord_cli/Cargo.toml -- migration generate -- --name=update_users -- --env=development --dry-run
cargo run --manifest-path=../../njord_cli/Cargo.toml -- migration generate -- --name=update_users -- --env=development
