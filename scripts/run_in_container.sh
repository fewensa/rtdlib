#!/bin/sh
#
#



BIN_PATH=$(dirname $(readlink -f $0))
SOURCE_PATH=$BIN_PATH/../


cd $SOURCE_PATH

cargo test --package rtdlib --test test_json test_box_object -- --exact


# D:\opt\scoop\persist\rustup\.cargo\registry\src\github.com-1ecc6299db9ec823\block-buffer-0.7.3
# /wind/opt/scoop/persist/rustup/.cargo/registry
# /wind/opt/scoop/persist/rustup/.cargo/git

