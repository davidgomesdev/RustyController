#!/usr/bin/env bash

# Colors

START=$(tput setaf 4)
INFO=$(tput setaf 6)
WARNING=$(tput setaf 3)
SUCCESS=$(tput setaf 2)
RESET=$(tput sgr0)

printf "$START* Running at $(date)$RESET\n"

# Start node_exporter

RUSTY_PATH=$(pwd)
NODE_EXPORTER_PATH=${NODE_EXPORTER_PATH:-../node_exporter}

if [ -f "$NODE_EXPORTER_PATH"/node_exporter ]; then
  (cd "$NODE_EXPORTER_PATH" && "$RUSTY_PATH"/server/scripts/run-node-exporter.sh)
else
  echo "${WARNING}Warning: node_exporter wasn't found in $(pwd)/$NODE_EXPORTER_PATH$RESET"
fi

# Start Grafana stack

printf "${INFO}* Launching Grafana stack...$RESET\n\n"

(cd server/docker && docker compose -f grafana.yaml up --wait -d) >> /var/log/rusty-controller/run-grafana-stack.log 2>&1

# Update and launch server

printf "${INFO}* Updating and launching server...$RESET\n\n"

(cp server/scripts/auto-update.sh /tmp/rusty-auto-update.sh && bash /tmp/rusty-auto-update.sh) >> /var/log/rusty-controller/auto-update.log 2>&1

# Update and launch non-adhoc plugins

printf "${INFO}* Updating and launching plugins...$RESET\n\n"

PLUGINS_PATH=${PLUGINS_PATH:-../RustyController-plugins}

mkdir -p /var/log/rusty-controller/plugins/ && (cd "$PLUGINS_PATH" && git pull && bash run-all.sh) >> /var/log/rusty-controller/plugins/run-all.log 2>&1

echo -e "${SUCCESS}* Finished!$RESET"
echo
