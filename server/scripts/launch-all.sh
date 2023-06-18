#!/usr/bin/env bash

# Start node_exporter

PLUGINS_PATH=${PLUGINS_PATH:-../RustyController-plugins}
NODE_EXPORTER_PATH=${NODE_EXPORTER_PATH:-../node_exporter}

(cd "$NODE_EXPORTER_PATH" && ./node_exporter) || echo "Warning: node_exporter wasn't found in $(pwd)/$NODE_EXPORTER_PATH"

# Start Grafana stack

(cd server/docker && docker compose -f grafana.yaml up --wait)

# Update and launch server

(cp server/scripts/auto-update.sh /tmp/rusty-auto-update.sh && bash /tmp/rusty-auto-update.sh) >> /var/log/rusty-controller/auto-update.log 2>&1

# Update and launch non-adhoc plugins

mkdir -p /var/log/rusty-controller/plugins/ && (cd "$PLUGINS_PATH" && git pull && bash run-all.sh) >> /var/log/rusty-controller/plugins/run-all.log 2>&1
