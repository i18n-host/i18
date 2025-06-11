#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
ROOT=${DIR%/*}
cd $ROOT/../conf/env
. password.env
set -xe

rm -rf conf.tar.gpg

FILE_LI="cf.env github.env upgrade.env upgrade/sk"

set +x
echo "$PASSWORD" | gpg --symmetric --batch --yes --passphrase-fd 0 -o $ROOT/conf.tar.gpg --cipher-algo AES256 <(tar -cvf - $FILE_LI)

# 更新本地配置
PASSWORD=$PASSWORD $DIR/confUnpack.sh
set -x
