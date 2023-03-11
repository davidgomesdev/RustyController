#!/usr/bin/env bash


ERROR=$(tput setaf 1)
INFO=$(tput setaf 6)
RESET=$(tput sgr0)

if [ ! -f ./grafana.yaml ]; then
    echo "${ERROR}Run this in the 'server/docker' directory!${RESET}"
    exit 1
fi

if [ "$(id -u)" -ne 0 ]; then
  echo "${ERROR}Run this as root!${RESET}"
  exit 1
fi

DOCKER_PATH=$(which docker)
CURRENT_DIR=$(pwd)

cat > /etc/systemd/system/docker-compose-RustyController-grafana.service << EOF
[Unit]
Description=RustyController Grafana Compose
Requires=docker.service
After=docker.service

[Service]
Restart=always
ExecStart=$DOCKER_PATH compose -f ${CURRENT_DIR}/grafana.yaml up -d
ExecStop=$DOCKER_PATH compose -f ${CURRENT_DIR}/grafana.yaml down -d

[Install]
WantedBy=default.target
EOF

systemctl enable docker-compose-RustyController-grafana.service || exit 1

echo
echo "${INFO}Installed successfully!${RESET}"
