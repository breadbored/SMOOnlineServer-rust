#!/usr/bin/env bash

echo "Building..."
cargo build

# Set up logging for the server
export RUST_BACKTRACE=full
DATETIME=$(date +'%Y-%m-%dT%T')
LOG_FILE="./$DATETIME.log"
exec 2> >( tee -a "$LOG_FILE" ) 3>&1 1>>"$LOG_FILE"

echo "Running..."
./target/debug/smo-rusty-online
