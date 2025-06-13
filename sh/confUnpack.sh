#!/usr/bin/env bash

if [ -z "$PASSWORD" ]; then
  echo "错误：请设置 PASSWORD 环境变量。" >&2
  exit 1
fi

DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR
set -xe

mkdir -p conf
cd conf
set +x
gpg --batch --yes --decrypt \
  --passphrase-fd 3 \
  ../conf.tar.gpg 3<<<"$PASSWORD" | tar -xvf -
set -x
