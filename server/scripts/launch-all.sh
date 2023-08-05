#!/usr/bin/env bash

# Colors

SUCCESS=$(tput setaf 2)
RESET=$(tput sgr0)

# Start node_exporter

RUSTY_PATH=$(pwd)
NODE_EXPORTER_PATH=${NODE_EXPORTER_PATH:-../node_exporter}

if [ -f "$NODE_EXPORTER_PATH"/node_exporter ]; then
  (cd "$NODE_EXPORTER_PATH" && "$RUSTY_PATH"/server/scripts/run-node-exporter.sh)
else
  echo "Warning: node_exporter wasn't found in $(pwd)/$NODE_EXPORTER_PATH"
fi

# Start Grafana stack

printf "Launching Grafana stack...\n\n"

(cd server/docker && docker compose -f grafana.yaml up --wait -d) >> /var/log/rusty-controller/run-grafana-stack.log 2>&1

# Update and launch server

printf "Updating and launching server...\n\n"

(cp server/scripts/auto-update.sh /tmp/rusty-auto-update.sh && bash /tmp/rusty-auto-update.sh) >> /var/log/rusty-controller/auto-update.log 2>&1

# Update and launch non-adhoc plugins

printf "Updating and launching plugins...\n\n"

PLUGINS_PATH=${PLUGINS_PATH:-../RustyController-plugins}

mkdir -p /var/log/rusty-controller/plugins/ && (cd "$PLUGINS_PATH" && git pull && bash run-all.sh) >> /var/log/rusty-controller/plugins/run-all.log 2>&1

echo -e "${SUCCESS}Finished!$RESET"
