#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

source ./sh/pid.sh

set -ex

cd src

exec watchexec \
  --shell=none \
  --project-origin . \
  -w . \
  --exts rs,toml \
  -r \
  -- "../test.sh"
