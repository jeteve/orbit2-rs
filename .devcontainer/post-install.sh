#! /bin/env bash

curl --proto '=https' https://sh.rustup.rs | sh  -s -- -y

source "$HOME/.cargo/env" && cargo install cargo-release
