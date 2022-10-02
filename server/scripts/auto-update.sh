#!/usr/bin/env bash

# Colors

SUCCESS=$(tput setaf 2)
WARNING=$(tput setaf 3)
INFO=$(tput setaf 6)
RESET=$(tput sgr0)

# Vars

REPO_URL="https://github.com/LegendL3n/RustyController"
RUSTY_HOME_DIR="$HOME/RustyController"
BINARY_PATH="$RUSTY_HOME_DIR/server/target/release/rusty_controller"
HASH_FILE="$RUSTY_HOME_DIR/current.sha256"

echo "$INFO* Running at $(date)$RESET"

cd "$HOME" || exit 1

build () {
    echo "$INFO* Building...$RESET"

    # This way we only pull the main branch
    git fetch origin main || exit 1
    git checkout FETCH_HEAD -B main || exit 1
    git reset --hard || exit 1
    git pull || exit 1
    cd server/ || exit 1

    cargo update -q || exit 1
    time cargo build --release -q || exit 1

    newest_hash=$(sha256sum "$BINARY_PATH" | gawk '{print $1}')

    echo "$newest_hash" > "$HASH_FILE"
    echo "$SUCCESS* Built successfully!$RESET"
    cd ..
}

launch () {
  echo "$SUCCESS* Launching built version...$RESET"
  . "$RUSTY_HOME_DIR/server/scripts/run-tmux-session.sh"
}

if [[ ! -d "$RUSTY_HOME_DIR" ]]; then
  echo "$WARNING* Rusty not found. Cloning...$RESET"
  git clone "$REPO_URL"

  build
  launch
  exit 0
fi

cd "$RUSTY_HOME_DIR" || exit 1

if [[ -f "$HASH_FILE" ]]; then
  if [ ! -f "$BINARY_PATH" ]; then
    echo "$INFO* Binary not found, building...$RESET"
    build
    launch
    exit 0
  fi

  current_hash=$(cat "$HASH_FILE")
  build
  if [[ "$current_hash" != "$newest_hash" ]]; then
      echo "$INFO* Built a new version!$RESET"
      launch
  else
      echo "$INFO* Version is already up-to-date.$RESET"
  fi
else
    echo "$WARNING* Couldn't find current hash. Updating to latest version anyway.$RESET"
    build
    launch
fi
