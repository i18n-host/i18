#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
. $DIR/env.sh
set -x
bun i

# windows 下面 bun x 会报错
updist=$DIR/node_modules/@3-/updist/mod.js

# $updist -h

. ./release.sh

{
  read -r NAME
  read -r VER
} <<<"$META"

find "/tmp/bin/$NAME/$VER" -mindepth 1 -maxdepth 1 -type d -print0 | while IFS= read -r -d '' dir; do
  $updist $NAME $VER $CONF/upgrade/sk $dir
done
