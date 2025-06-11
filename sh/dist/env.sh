CONF=${DIR%/*/*}/conf
cd $CONF
set -a
. cf.env
. github.env
. upgrade.env
set +a
cd $DIR
