#!/bin/bash
COMMAND="cargo run"
cargo build -u --manifest-path $PWD/message_bus/Cargo.toml
gnome-terminal --working-directory=$PWD/message_bus -- bash -c "$COMMAND; bash"
