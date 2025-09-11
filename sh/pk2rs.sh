#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

UPGRADE_MOD=../src/upgrade
mkdir -p $UPGRADE_MOD

if [ ! -f "$UPGRADE_MOD/mod.rs" ]; then
  echo -e "mod pk;\npub use pk::PK;" >$UPGRADE_MOD/mod.rs
fi

RS=$UPGRADE_MOD/pk.rs
echo -e "// gen by ./sh/pk2rs.sh\n\npub const PK: [u8;32] = [" >$RS

hexdump -v -e '1/1 "    0x%02x,\n"' ../../conf/env/upgrade/pk >>$RS

echo "];" >>$RS

echo $RS
