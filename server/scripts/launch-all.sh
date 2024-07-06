#!/usr/bin/env bash

# Colors

START=$(tput setaf 4)
INFO=$(tput setaf 6)
SECTION=$(tput setaf 5)
WARNING=$(tput setaf 3)
SUCCESS=$(tput setaf 2)
RESET=$(tput sgr0)

# Get params

show_usage () {
  echo "Parameters:"
  echo "-g: launch Grafana stack"
  . server/scripts/auto-update.sh -h
}

while getopts "blh" opt; do
  case ${opt} in
    g)
      LAUNCH_GRAFANA="y"
      ;;
    h)
      show_usage
      exit 0
      ;;
    ?)
      echo "Invalid option: -${OPTARG}."
      show_usage
      exit 1
      ;;
  esac
done

printf "$START* Running at $(date)$RESET\n"

set -e

# Start Grafana stack

if [[ "$LAUNCH_GRAFANA" == "y" ]]; then
  GRAFANA_COMPOSE_OVERRIDE=${GRAFANA_COMPOSE_OVERRIDE:-base}

  printf "${SECTION}* Launching Grafana stack...$RESET\n\n"

  # Here we only care about stderr
  (cd server/docker && docker compose -f grafana.yaml -f "grafana.$GRAFANA_COMPOSE_OVERRIDE.yaml" up --wait -d) > /dev/null
fi

# Update and launch server

printf "\n${SECTION}* Updating and launching server...$RESET\n\n"

# Forwards all arguments passed (for args build and launch always)
cp server/scripts/auto-update.sh /tmp/rusty-auto-update.sh && bash /tmp/rusty-auto-update.sh "$@"

# Update and launch non-adhoc plugins

printf "\n${SECTION}* Updating and launching plugins...$RESET\n\n"

PLUGINS_PATH=${PLUGINS_PATH:-../RustyController-plugins}

mkdir -p /var/log/rusty-controller/plugins/ && cd "$PLUGINS_PATH"

# Run plugins only if outdated
ARE_PLUGINS_UPDATED=$(git remote update && git status -uno | grep -q 'Your branch is behind' && git pull)

if [[ ! $ARE_PLUGINS_UPDATED ]]; then
  printf "\n${INFO}Plugins are outdated...$RESET\n\n"
  bash run-all.sh
else
  tmux has-session -t "RustyController plugins" 2>/dev/null
  if [ $? != 0 ]; then
    printf "\n${INFO}Plugins are not running, running...$RESET\n\n"
    bash run-all.sh
  else
    printf "\n${INFO}Plugins are up-to-date$RESET\n\n"
  fi
fi

echo -e "\n${SUCCESS}* Finished! (at $(date))$RESET"
echo

cd ../RustyController
