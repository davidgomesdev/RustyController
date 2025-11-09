#!/usr/bin/env bash

# Colors

START=$(tput setaf 4)
SUCCESS=$(tput setaf 2)
WARNING=$(tput setaf 3)
INFO=$(tput setaf 6)
RESET=$(tput sgr0)

# Vars

REPO_URL="https://github.com/DavidGomesDev/RustyController"
RUSTY_HOME_DIR="$HOME/RustyController"
BINARY_PATH="$RUSTY_HOME_DIR/server/target/release/rusty_controller"
HASH_FILE="$RUSTY_HOME_DIR/current.sha256"

# Get params

show_usage () {
  echo "Parameters:"
  echo "-b: build binary (instead of downloading the latest release)"
  echo "-l: only launch (useful on reboot since network may not be available yet)"
}

while getopts "blh" opt; do
  case ${opt} in
    b)
      BUILD_BINARY="y"
      ;;
    l)
      LAUNCH_ONLY="y"
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

echo "$START* Updating server at $(date)$RESET"

cd "$HOME" || exit 1

download_latest () {
  # Get arch (and trim output)
  arch=$(uname -m | xargs echo -n)

  mkdir -p target/release || exit 1
  wget -q "$REPO_URL/releases/latest/download/server-$arch" -O "$BINARY_PATH" || exit 1
  chmod +x "$BINARY_PATH" || exit 1

  newest_hash=$(sha256sum "$BINARY_PATH" | gawk '{print $1}')

  echo "$INFO* Downloaded latest release binary$RESET"
}

build () {
  echo
  echo "$INFO* Updating crates...$RESET"
  echo

  cargo update -q || exit 1

  echo "$INFO* Build...$RESET"
  echo

  time cargo build --release -q || exit 1

  newest_hash=$(sha256sum "$BINARY_PATH" | gawk '{print $1}')

  echo "$SUCCESS* Built successfully!$RESET"
}

update () {
    echo "$INFO* Checking out main...$RESET"
    echo
    
    git reset --hard > /dev/null || exit 1

    # This way we only pull the main branch
    git fetch origin main > /dev/null || exit 1
    git switch main > /dev/null || exit 1
    git pull > /dev/null || exit 1

    cd server/ || exit 1

    if [[ "$BUILD_BINARY" == "y" ]]; then
      build
    else
      download_latest
    fi

    cd ..
}

launch () {
  echo "$SUCCESS* Launching...$RESET"
  . "$RUSTY_HOME_DIR/server/scripts/run-server.sh"
}

if [[ ! -d "$RUSTY_HOME_DIR" ]]; then
  echo "$WARNING* Rusty repo not found. Cloning...$RESET"
  git clone "$REPO_URL"

  update
  launch

  echo "$START* Finished installing at $(date)$RESET"
  exit 0
fi

cd "$RUSTY_HOME_DIR" || exit 1

if [[ "$LAUNCH_ONLY" == "y" ]]; then
  printf "\n${INFO}Launching due to only launch flag$RESET\n\n"
  launch
  exit 0
fi

if [[ -f "$HASH_FILE" ]]; then
  current_hash=$(cat "$HASH_FILE")
  update
  if [[ "$current_hash" != "$newest_hash" ]]; then
      newest_hash=$(sha256sum "$BINARY_PATH" | gawk '{print $1}')
    
      echo "$newest_hash" > "$HASH_FILE"
      echo "$INFO* There is a new version!$RESET"
      launch
  else
      echo "$INFO* Version is already up-to-date.$RESET"

      tmux has-session -t "RustyController" 2>/dev/null
      if [ $? != 0 ]; then
        printf "\n${INFO}Server is not running$RESET\n\n"
        launch
      fi
  fi
else
    echo "$WARNING* Couldn't find current hash. Updating to latest version anyway.$RESET"
    update
    launch

    newest_hash=$(sha256sum "$BINARY_PATH" | gawk '{print $1}')

    echo "$newest_hash" > "$HASH_FILE"
fi

echo "$START* Finished auto-update at $(date)$RESET"
