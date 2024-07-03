# EP2: Run Ethereum Node Using Reth


### Install reth
```shell
wget https://github.com/paradigmxyz/reth/releases/download/v1.0.0/reth-v1.0.0-x86_64-unknown-linux-gnu.tar.gz
tar xfz reth-v1.0.0-x86_64-unknown-linux-gnu.tar.gz
mv reth /usr/local/bin/ 
```

### Install lighthouse
```shell
wget https://github.com/sigp/lighthouse/releases/download/v5.2.1/lighthouse-v5.2.1-x86_64-unknown-linux-gnu-portable.tar.gz
tar xfz lighthouse-v5.2.1-x86_64-unknown-linux-gnu-portable.tar.gz
mv lighthouse /usr/local/bin/
```

### Run reth and lightouse in a new tmux session
```shell
tmux new -s eth

reth node --full --chain=holesky --datadir=/eth/reth --http --http.addr 0.0.0.0 --http.port 8545 --http.api "eth,net,web3"
 
lighthouse bn --network=holesky --datadir=/eth/lighthouse --checkpoint-sync-url=https://holesky.beaconstate.info \
    --execution-endpoint=http://localhost:8551 --execution-jwt=/eth/reth/jwt.hex --disable-deposit-contract-sync 
```

### Check the RPC node via cast, curl
```shell
# `test` points to a real IP address in /etc/hosts
export ETH_RPC_URL=http://test:8545 
cast chain-id
cast block-number
cast gas-price
cast rpc eth_syncing
curl -X POST --header "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' http://test:8545
```

## Links
- https://github.com/paradigmxyz/reth
- https://reth.rs
- https://lighthouse-book.sigmaprime.io/
- https://book.getfoundry.sh/cast/
- https://ethereum.org/en/developers/docs/apis/json-rpc/
