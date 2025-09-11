#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex


if [ $# -eq 0 ]; then
  set -- -w ./test/test
fi

exec cargo run -- $@
