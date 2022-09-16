#!/usr/bin/env bash

# Colors

SUCCESS=$(tput setaf 2)
WARNING=$(tput setaf 3)
INFO=$(tput setaf 6)
RESET=$(tput sgr0)

# Vars

REPO_URL="https://github.com/LegendL3n/RustyController"
LATEST_URL="$REPO_URL/releases/download/latest/rusty_controller"
RUSTY_HOME_DIR="$HOME/RustyController"
HASH_FILE="$RUSTY_HOME_DIR/rusty_controller.sha256"

cd /tmp || exit 1
rm rusty_controller 2> /dev/null

wget -q "$LATEST_URL" || exit
newest_hash=$(sha256sum "rusty_controller" | gawk '{print $1}')

rm rusty_controller

update () {
    echo "$INFO* Updating...$RESET"
    rm -rf ./RustyController 2> /dev/null

    git clone "$REPO_URL" || exit
    cd RustyController/server/ || exit 1

    cargo build --release || exit
    mv target/release/rusty_controller "$RUSTY_HOME_DIR/rusty_controller" || exit

    echo "$newest_hash" > "$HASH_FILE"
    echo "$SUCCESS* Updated successfully!$RESET"
}

if [[ ! -f "$RUSTY_HOME_DIR/rusty_controller" ]]; then
  echo "$WARNING* Rusty binary not found.$RESET"
  update
  exit 0
fi

if [[ -f "$RUSTY_HOME_DIR/rusty_controller.sha256" ]]; then
    if [[ "$newest_hash" != $(cat "$HASH_FILE") ]]; then
        echo "$INFO* Found a new version!$RESET"
        update
    else
        echo "$INFO* Version is already up-to-date.$RESET"
        exit 0
    fi
else
    echo "$WARNING* Couldn't find current hash. Updating to latest version anyway.$RESET"
    update
fi
