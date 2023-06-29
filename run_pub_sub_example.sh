#!/bin/bash
COMMAND="cargo run"
cargo build -u --manifest-path $PWD/message_bus/Cargo.toml
cargo build -u --manifest-path $PWD/message_bus_client/Cargo.toml
gnome-terminal --working-directory=$PWD/message_bus -- bash -c "$COMMAND; bash"
gnome-terminal --working-directory=$PWD/message_bus_client  -- bash -c "$COMMAND --example subscriber_client2; bash"
gnome-terminal --working-directory=$PWD/message_bus_client -- bash -c "$COMMAND --example publisher_client1; bash"
