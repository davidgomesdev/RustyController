#!/usr/bin/env bash

tmux kill-session -t RustyController 2>/dev/null && echo "Killed existing tmux session"
tmux new-session -d -s RustyController 'cp ./server/target/debug/rusty_controller /tmp/rusty_controller && export RUST_LOG=info,rusty_controller=debug; /tmp/rusty_controller'
echo "Started new tmux session for RustyController"
