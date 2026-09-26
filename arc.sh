#!/usr/bin/env bash

cmd="$1"

if [[ "$cmd" == "help" || "$cmd" == "run" ]]; then
    cargo run -q -- "$@"
else
    echo "[Error] Unknown command: '$cmd'."
    echo "--> Use 'help' for the list of valid commands."
    exit 1
fi