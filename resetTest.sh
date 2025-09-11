#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cd test/test/.i18n
rm -rf hash && rm -f src.yml

$DIR/test.sh
