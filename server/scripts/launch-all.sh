#!/usr/bin/env bash

# Colors

START=$(tput setaf 4)
SECTION=$(tput setaf 5)
WARNING=$(tput setaf 3)
SUCCESS=$(tput setaf 2)
RESET=$(tput sgr0)

printf "$START* Running at $(date)$RESET\n"

# Start node_exporter

GRAFANA_COMPOSE_OVERRIDE=${GRAFANA_COMPOSE_OVERRIDE:-base}

# Start Grafana stack

printf "${SECTION}* Launching Grafana stack...$RESET\n\n"

# Here we only care about stderr
(cd server/docker && docker compose -f grafana.yaml -f "grafana.$GRAFANA_COMPOSE_OVERRIDE.yaml" up --wait -d) > /dev/null

# Update and launch server

printf "\n${SECTION}* Updating and launching server...$RESET\n\n"

# Forwards all arguments passed (for args build and launch always)
cp server/scripts/auto-update.sh /tmp/rusty-auto-update.sh && bash /tmp/rusty-auto-update.sh "$@"

# Update and launch non-adhoc plugins

printf "\n${SECTION}* Updating and launching plugins...$RESET\n\n"

PLUGINS_PATH=${PLUGINS_PATH:-../RustyController-plugins}

# Run plugins only if outdated
mkdir -p /var/log/rusty-controller/plugins/ && cd "$PLUGINS_PATH" && git remote update && git status -uno | grep -q 'Your branch is behind' && git pull && bash run-all.sh

echo -e "\n${SUCCESS}* Finished! (at $(date))$RESET"
echo
