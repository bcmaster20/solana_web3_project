
# Staking 

## Check wallet
- Edit Anchor.toml
```
[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"
```

## Test
```
anchor test -- --features local-testing,test-id
```

