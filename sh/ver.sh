#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR/..
set -ex

cargo v patch -y

#  git add . && git commit -m "."
