#!/bin/bash
CARGO_RUN_COMMAND="cargo run --release"
CARGO_BUILD_COMMAND="cargo build --release"
$CARGO_BUILD_COMMAND --manifest-path=$PWD/message_bus/Cargo.toml
$CARGO_BUILD_COMMAND --manifest-path=$PWD/message_bus_client/Cargo.toml 


tmux new-session -d\
    "cd $PWD/message_bus && $CARGO_RUN_COMMAND; bash" \;\
    split-window -h "cd $PWD/message_bus_client/ && $CARGO_RUN_COMMAND --example subscriber_client2; bash" \;\
    select-pane -t 1 \;\
    split-window -v "cd $PWD/message_bus_client/ && $CARGO_RUN_COMMAND  --example publisher_client1; bash" \;\
    attach\;
