#!/bin/bash
# -----------------------------------------------------------------------------
# Script: run.sh
# Description: Run new migration schema changes with SQLite support.
# Author: Marcus Cvjeticanin
# Date: December 15, 2023
# -----------------------------------------------------------------------------

cargo run --manifest-path=../../njord_cli/Cargo.toml -- migration run --env=development --log-level=debug
