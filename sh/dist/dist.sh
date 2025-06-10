#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}

CONF=${DIR%/*/*}/conf
cd $CONF
set -a
. cf.env
. github.env
. upgrade.env
set +a

set -x
cd $DIR
bun i

case "$(uname -s)" in
MINGW*)
  bash --version
  which env

  mkdir -p /tmp/coreutils
  cd /tmp/coreutils
  zipfile=coreutils-0.1.0-x86_64-pc-windows-msvc.zip
  curl -OL https://github.com/uutils/coreutils/releases/download/0.1.0/$zipfile
  unzip -j $zipfile
  chmod +x coreutils.exe
  mv coreutils.exe $(which env)

  cd $DIR
  bun x updist -h
  ;;
esac

. ./release.sh

{
  read -r NAME
  read -r VER
} <<<"$META"

find "/tmp/bin/$NAME/$VER" -mindepth 1 -maxdepth 1 -type d -print0 | while IFS= read -r -d '' dir; do
  bun x updist $NAME $VER $CONF/upgrade/sk $dir
done
