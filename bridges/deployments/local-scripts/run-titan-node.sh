#!/bin/bash

# Run a development instance of the titan Substrate bridge node.
# To override the default port just export titan_PORT=9955
#
# Note: This script will not work out of the box with the bridges
# repo since it relies on a diamond binary.

titan_PORT="${titan_PORT:-9955}"

RUST_LOG=runtime=trace,runtime::bridge=trace \
./target/debug/diamond --chain=titan-dev --alice --tmp \
    --rpc-cors=all --unsafe-rpc-external --unsafe-ws-external \
    --port 33044 --rpc-port 9934 --ws-port $titan_PORT \
