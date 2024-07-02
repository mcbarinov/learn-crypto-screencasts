# EP1: Run Ethereum Node Using Geth


### Install geth
```shell
add-apt-repository -y ppa:ethereum/ethereum
apt-get update
apt-get install ethereum -y
```

### Install lighthouse
```shell
wget https://github.com/sigp/lighthouse/releases/download/v5.2.1/lighthouse-v5.2.1-x86_64-unknown-linux-gnu-portable.tar.gz
tar xfz lighthouse-v5.2.1-x86_64-unknown-linux-gnu-portable.tar.gz
mv lighthouse /usr/local/bin/
```

### Run geth and lighthouse in a new tmux session
```shell
geth --holesky --datadir=/eth/geth --authrpc.addr=localhost --authrpc.port=8551 --authrpc.jwtsecret=/eth/geth/jwtsecret \
    --http --http.api=eth,net --http.addr=0.0.0.0 --http.vhosts=* --http.port=8545
    
lighthouse bn --network holesky --datadir /eth/lighthouse --disable-deposit-contract-sync --execution-jwt=/eth/geth/jwtsecret \
  --checkpoint-sync-url=https://holesky.beaconstate.info --execution-endpoint=http://localhost:8551   
```

### Check the RPC node
```shell
# `test` points to the real IP address of the node in /etc/hosts
curl --data '{"method":"eth_blockNumber","id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST test:8545
curl --data '{"method":"eth_syncing","id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST test:8545
curl --data '{"method":"net_peerCount","id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST test:8545
```

## Links
- https://github.com/ethereum/go-ethereum
- https://github.com/sigp/lighthouse
- https://geth.ethereum.org/docs
- https://github.com/eth-clients/holesky


