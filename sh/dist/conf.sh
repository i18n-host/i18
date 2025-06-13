#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cd ../..
if [ ! -d "conf" ]; then
  set +x
  . dist.conf
  cd ..
  git clone --depth=1 -b dev $CONF_REPO conf
  set -x
fi
