#!/bin/bash
# -----------------------------------------------------------------------------
# Script: rollback.sh
# Description: Rollback migration changes to specified change with SQLite support.
# Author: Marcus Cvjeticanin
# Date: December 15, 2023
# -----------------------------------------------------------------------------

cargo run --manifest-path=../../njord_cli/Cargo.toml -- migration rollback --to=00000000000000
