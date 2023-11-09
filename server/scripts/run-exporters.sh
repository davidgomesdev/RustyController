#!/usr/bin/env bash

LOG_DIRECTORY="/var/log/rusty-controller/prometheus-exporters"
mkdir -p "$LOG_DIRECTORY"

tmux kill-session -t prometheus-exporters 2>/dev/null && echo "Killed existing Prometheus Exporters tmux session"

tmux new-session -d -s "Prometheus Exporters"

tmux new-window -n "node_exporter" -c "$(pwd)" "./node_exporter 2>&1 | tee $LOG_DIRECTORY/node_exporter.log"
echo "Started new tmux session for node_exporter"

tmux new-window -d -n "process-exporter" -c "$(pwd)" "process-exporter 2>&1 | tee $LOG_DIRECTORY/process-exporter.log"
echo "Started new tmux session for process-exporter"

