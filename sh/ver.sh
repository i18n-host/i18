#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR/..
set -ex
git add .
cargo v patch -y
git push
ver=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version')
git push origin v$ver
