#!/usr/bin/env bash

. ${DEMO_MAGIC_PATH}

# TYPE_SPEED=20

DEMO_PROMPT="${GREEN}➜ ${CYAN}\W ${COLOR_RESET}"

# DEMO_CMD_COLOR=$BLACK

REDE_PATH=../target/debug/rede
if [ ! -f ${REDE_PATH} ]; then
  cd .. && cargo build -p rede
fi

rede() {
  ${REDE_PATH} "$@"
}

clear

pe "rede --q example"
pei "bat example.toml"
PROMPT_TIMEOUT=2
wait
pei "clear"
PROMPT_TIMEOUT=0

pei "rede run example"

p ""
