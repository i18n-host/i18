#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -e

cargo metadata --no-deps --format-version=1 | jq -br '.packages[0].name,.packages[0].version,.target_directory'
