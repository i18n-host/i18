#!/usr/bin/env bash

case "$(uname -s)" in
Darwin)
  if ! command -v realpath &>/dev/null; then
    brew install coreutils || true
  fi
  ;;
esac

DIR=$(dirname "${BASH_SOURCE[0]}")

cd $DIR

set -ex

META=$(./meta.sh)
./cross.sh $META
