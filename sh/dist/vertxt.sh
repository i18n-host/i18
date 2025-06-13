#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
. $DIR/env.sh
set -x
./conf.sh
if ! command -v vertxt &>/dev/null; then
  bun i -g @3-/vertxt
fi

{
  read -r NAME
  read -r VER
} <<<$(./meta.sh)

bun x -b vertxt $NAME $VER 10
