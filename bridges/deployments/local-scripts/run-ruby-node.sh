#!/bin/bash

# Run a development instance of the ruby Substrate bridge node.
# To override the default port just export ruby_PORT=9945
#
# Note: This script will not work out of the box with the bridges
# repo since it relies on a diamond binary.

ruby_PORT="${ruby_PORT:-9944}"

RUST_LOG=runtime=trace,runtime::bridge=trace \
./target/debug/diamond --chain=ruby-dev --alice --tmp \
    --rpc-cors=all --unsafe-rpc-external --unsafe-ws-external \
    --port 33033 --rpc-port 9933 --ws-port $ruby_PORT \
