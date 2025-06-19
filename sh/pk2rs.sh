#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

RS=../src/upgrade.rs
echo "pub const PK: [u8;32] = [" >$RS

hexdump -v -e '1/1 "    0x%02x,\n"' ../../conf/env/upgrade/pk >>$RS

echo "];" >>$RS

echo $RS
