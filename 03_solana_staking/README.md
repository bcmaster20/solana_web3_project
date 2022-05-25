
## Check wallet
- Edit Anchor.toml
```
[provider]
cluster = "localnet"
wallet = "/home/bmstart/work/solana/keys/alice.json"
```

## Test
```
anchor test -- --features local-testing,test-id
```

