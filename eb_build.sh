#!/usr/bin/env bash

case "$1" in
  "musl" )
    ;;
  "gnu" )
    ;;
   * )
    echo 'musl or gnu'
    exit 1
    ;;
esac

export CC_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-gcc
export CXX_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-g++
export AR_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-musl-gcc

export CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc
export CXX_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-g++
export AR_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc

unset RUSTC_WRAPPER
rustup target add "x86_64-unknown-linux-$1"
cargo build --release --target="x86_64-unknown-linux-$1"

#
# make eb.zip
#
if [ ! -d  "./elasticbeanstalk/$1/PlanningPoker" ]; then
  mkdir -p "./elasticbeanstalk/$1/PlanningPoker"
fi
cp ./target/x86_64-unknown-linux-$1/release/planning_poker "./elasticbeanstalk/$1/PlanningPoker/"
rm "./eb_planning_poker_$1.zip"
pushd "./elasticbeanstalk/$1" || exit 2
zip -r "../../eb_planning_poker_$1.zip" *
popd || exit 3
NOW=$(date +%Y%m%d_%H%M%S)
cp "eb_planning_poker_$1.zip" "eb_planning_poker_$1_${NOW}.zip"
