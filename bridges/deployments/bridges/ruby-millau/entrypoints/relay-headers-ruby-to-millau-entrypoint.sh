#!/bin/bash
set -xeu

sleep 20
curl -v http://millau-node-alice:9933/health
curl -v https://ruby-rpc.diamond.io:443/health

/home/user/substrate-relay init-bridge rubyToMillau \
	--source-host ruby-rpc.diamond.io \
	--source-port 443 \
	--source-secure \
	--target-host millau-node-alice \
	--target-port 9944 \
	--target-signer //George

# Give chain a little bit of time to process initialization transaction
sleep 6
/home/user/substrate-relay relay-headers rubyToMillau \
	--source-host ruby-rpc.diamond.io \
	--source-port 443 \
	--source-secure \
	--target-host millau-node-alice \
	--target-port 9944 \
	--target-signer //George \
	--prometheus-host=0.0.0.0
