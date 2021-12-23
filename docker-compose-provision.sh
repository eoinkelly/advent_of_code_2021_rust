#!/usr/bin/env bash

set -x
set -e

apt-get -y update && apt-get -y upgrade

apt-get -y install curl vim git build-essential valgrind

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# make linux build into diff dir than macos
echo "export CARGO_TARGET_DIR=/data/linux_target" >> ~/.bashrc