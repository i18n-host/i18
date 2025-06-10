#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR/../conf/env
. password.env
set -xe

rm -rf conf.tar.gpg

FILE_LI="cf.env github.env upgrade.env upgrade/sk"

set +x
echo "$PASSWORD" | gpg --symmetric --batch --yes --passphrase-fd 0 -o $DIR/conf.tar.gpg --cipher-algo AES256 <(tar -cvf - $FILE_LI)
set -x
