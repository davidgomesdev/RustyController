#!/usr/bin/env bash

tmux kill-session -t RustyController 2>/dev/null && echo "Killed existing tmux session"
# Runs from /tmp for the update to only shutdown when it's starting the new version. (similar to a staging release)
tmux new-session -d -s RustyController -c "$(pwd)" 'cp ./server/target/release/rusty_controller /tmp/running_rusty_controller && export RUST_LOG=info,rusty_controller=debug; /tmp/running_rusty_controller'
echo "Started new tmux session for RustyController"
