#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

rm -rf ./test/.i18n/hash

if [ $# -eq 0 ]; then
  set -- -w ./test
fi

exec cargo run -- $@
