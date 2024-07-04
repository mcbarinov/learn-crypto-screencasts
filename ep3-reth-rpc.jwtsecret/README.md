# EP3: How to Run Reth with a JWT Secret for Authenticating Regular RPC Servers (HTTP, WS)

### Install reth
```shell
wget https://github.com/paradigmxyz/reth/releases/download/v1.0.0/reth-v1.0.0-x86_64-unknown-linux-gnu.tar.gz
tar xfz reth-v1.0.0-x86_64-unknown-linux-gnu.tar.gz
mv reth /usr/local/bin/ 
```

### Run reth with --rpc.jwtsecret 
```shell
export JWT_SECRET=0000000000000000000000000000000000000000000000000000000000000abc
reth node --full --chain=holesky --datadir=/eth/reth --rpc.jwtsecret=$JWT_SECRET --http --http.addr 0.0.0.0 \
  --http.port 8545 --http.api "eth,net,web3" 
```

### Send a request without JWT
```shell
curl -X POST --header "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":1}' http://test:8545

# Authorization header is missing or invalid
```

### Send a request with JWT
```shell
# get a new token with `cargo run`. Token is valid for 1 minute.
export TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE3MjAwMTU1NDZ9.SnzRjINBnH7xjVenCxdD5v2pr5lEM_lXWVJxrNDdLMA
curl http://test:8545 \
--data '{"method": "net_peerCount", "params": [], "id": 1, "jsonrpc": "2.0"}' \
--header 'Content-Type: application/json' \
--header "Authorization: Bearer $TOKEN"

# {"jsonrpc":"2.0","result":"0x2","id":1}
```


### Links
- https://reth.rs/cli/reth/node.html
- https://github.com/mcbarinov/learn-rust-screencasts/tree/main/ep1-jsonwebtoken