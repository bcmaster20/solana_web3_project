# MetaPlex
- MetaPlex Docs(https://docs.metaplex.com/storefront/installation)

## Local setup
```
$ git clone https://github.com/metaplex-foundation/metaplex.git
$ cd metaplex/js && yarn install && yarn bootstrap
$ yarn start
```

## Init store
In the Store configuration section on the store page click on the Copy button and paste in the .env file in js/packages/web
```
REACT_APP_STORE_OWNER_ADDRESS_ADDRESS=uPMCGZB4NTtQAQ6eLPUhLtRtJTHyF8jZfb8Nma4q6UJ
```

# Candy Machine V2
- [Candy Machine V2 Docs](https://docs.metaplex.com/candy-machine-v2/configuration)

## Configuration
- ~metaplex/js/packages/cli/CandyV2-config.json
```
  "price": 0.1,
  "number": 10,
  "solTreasuryAccount": "uPMCGZB4NTtQAQ6eLPUhLtRtJTHyF8jZfb8Nma4q6UJ",
  "splTokenAccount": null,
  "splToken": null,
  "goLiveDate": "25 Dec 2022 00:00:00 GMT",

```

- Copy png and metadata to ~metaplex/js/s_assets
```
$ cd ~/metaplex/js/
$ ts-node ~/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts verify_assets ./s_assets

ts-node /home/jaylee/01_project/solana_web3_project/06_metaplex/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts upload -e devnet -k ~/.config/solana/id.json -cp CandyV2-Config.json ./s_assets
```


