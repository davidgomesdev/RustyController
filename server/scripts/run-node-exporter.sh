#!/usr/bin/env bash

tmux kill-session -t node_exporter 2>/dev/null && echo "Killed existing node_exporter tmux session"
tmux new-session -d -s node_exporter -c "$(pwd)" './node_exporter'
echo "Started new tmux session for node_exporter"
