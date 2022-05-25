# Staking Tokan
## Requirement
```
So basically user functions:
stake, unstake, restake, withdraw
```

## Project Install
```
$ git clone 
$ yarn install
$ anchor build
$ anchor keys list
Program ID: xxx
Note: Copy Program ID, Past to lib.rs and Anchor.toml
```

## Check wallet
- Edit Anchor.toml
```
[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"
```

## Run Test Command
```
anchor test -- --features local-testing,test-id
```

## Run Test Result
![](https://github.com/bcmaster20/solana_web3_project/blob/main/03_solana_staking/StakingTestResult.png)
