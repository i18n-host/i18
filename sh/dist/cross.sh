#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
ROOT=${DIR%/*/*}

if [ -z "$3" ]; then
  echo "USAGE : $0 project_name version target_dir"
  exit 1
else
  PROJECT=$1
  VER=$2
  OUTDIR=$3
fi

set -ex

build="build" # -Z unstable-options

unameOut="$(uname -s)"

case "${unameOut}" in
MINGW*)
  choco install activeperl nasm coreutils &
  RUSTFLAGS="$RUSTFLAGS -C target-feature=+crt-static"
  TARGET_LI=$(rustc -vV | awk '/host/ { print $2 }')
  ;;
Linux)
  # sudo apt-get install -y pkg-config libssl-dev
  build="zigbuild"
  if ! command -v cargo-zigbuild &>/dev/null; then
    cargo install cargo-zigbuild
  fi
  TARGET_LI=(aarch64-unknown-linux-gnu aarch64-unknown-linux-musl x86_64-unknown-linux-gnu x86_64-unknown-linux-musl)
  ;;
Darwin)
  TARGET_LI=$(rustc -vV | awk '/host/ { print $2 }')
  if ! command -v protoc &>/dev/null; then
    brew install protobuf
  fi
  ;;
esac

for target in ${TARGET_LI[@]}; do
  rm -rf $OUTDIR/$target/release
  ./target.sh $target &
done

wait

NAME=$(echo $PROJECT | sed 's/\./-/g')

build="cargo $build -Z build-std=std,panic_abort --release --target"
# build="cargo $build -p $NAME -Z build-std=std,panic_abort --release --target"

. RUSTFLAGS.sh

for target in ${TARGET_LI[@]}; do
  if [[ "$target" == *"aarch"* ]]; then
    feature="+neon"
  else
    feature="+sse2"
  fi
  RUSTFLAGS="$RUSTFLAGS -C target-feature=$feature" $build $target
done

if [[ "$unameOut" == MINGW* ]]; then
  # https://github.com/briansmith/ring/issues/1514
  target=aarch64-pc-windows-msvc
  TARGET_LI="$TARGET_LI $target"
  # Get Visual Studio installation directory
  VSINSTALLDIR=$(vswhere.exe -latest -requires Microsoft.VisualStudio.Component.VC.Llvm.Clang -property installationPath)/VC
  LLVM_ROOT=$VCINSTALLDIR/Tools/Llvm/x64
  export PATH=$PATH:/usr/local/bin/nasm:$LLVM_ROOT/bin
  ./target.sh $target
  $build $target
fi

bindir=/tmp/bin/$PROJECT/$VER
rm -rf $bindir

for target in ${TARGET_LI[@]}; do
  todir=$bindir/$target
  mkdir -p $todir
  find "$OUTDIR/$target/release" -maxdepth 1 -type f -perm 755 -print0 | xargs -0 sh -c 'mv "$@" "$0"' "$todir"
done
