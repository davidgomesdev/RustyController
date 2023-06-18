#!/usr/bin/env bash

# Start node_exporter

RUSTY_PATH=$(pwd)
NODE_EXPORTER_PATH=${NODE_EXPORTER_PATH:-../node_exporter}

if [ -f "$NODE_EXPORTER_PATH"/node_exporter ]; then
  (cd "$NODE_EXPORTER_PATH" && "$RUSTY_PATH"/server/scripts/run-node-exporter.sh)
else
  echo "Warning: node_exporter wasn't found in $(pwd)/$NODE_EXPORTER_PATH"
fi

# Start Grafana stack

(cd server/docker && docker compose -f grafana.yaml up --wait)

# Update and launch server

(cp server/scripts/auto-update.sh /tmp/rusty-auto-update.sh && bash /tmp/rusty-auto-update.sh) >> /var/log/rusty-controller/auto-update.log 2>&1

# Update and launch non-adhoc plugins

PLUGINS_PATH=${PLUGINS_PATH:-../RustyController-plugins}

mkdir -p /var/log/rusty-controller/plugins/ && (cd "$PLUGINS_PATH" && git pull && bash run-all.sh) >> /var/log/rusty-controller/plugins/run-all.log 2>&1
