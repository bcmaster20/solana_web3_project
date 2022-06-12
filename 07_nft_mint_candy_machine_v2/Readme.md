# NFT MINT by CandyMachine V2

## Project Init
```
$ git clone https://github.com/buildspace/nft-drop-starter-project
$ cd app && npm install && npm run start
$ mkdir assets (copy 0~3 json and png file to assets )
```

## Install Metaplex in home directory
```
git clone -b v1.1.1 https://github.com/metaplex-foundation/metaplex.git
yarn install --cwd ~/metaplex/js/
ts-node ~/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts --version
```

## Configure your candy machine
```
{
    "price": 0.1,
    "number": 3,
    "gatekeeper": null,
    "solTreasuryAccount": "<YOUR WALLET ADDRESS>",
    "splTokenAccount": null,
    "splToken": null,
    "goLiveDate": "05 Jan 2021 00:00:00 GMT",
    "endSettings": null,
    "whitelistMintSettings": null,
    "hiddenSettings": null,
    "storage": "arweave",
    "ipfsInfuraProjectId": null,
    "ipfsInfuraSecret": null,
    "awsS3Bucket": null,
    "noRetainAuthority": false,
    "noMutable": false
}
```
## Upload the NFTs and create your candy machine
```
$ ts-node ~/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts upload -e devnet -k ~/.config/solana/id.json -cp config.json ./assets

$ ts-node ~/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts verify_upload -e devnet -k ~/.config/solana/id.json

wallet public key: uPMCGZB4NTtQAQ6eLPUhLtRtJTHyF8jZfb8Nma4q6UJ
Key size 3
Looking at key  0
Looking at key  1
Looking at key  2
uploaded (3) out of (3)
ready to deploy!


$ ts-node ~/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts update_candy_machine -e devnet -k ~/.config/solana/id.json -cp config.json
wallet public key: uPMCGZB4NTtQAQ6eLPUhLtRtJTHyF8jZfb8Nma4q6UJ
update_candy_machine finished oZF6kJvUNCtezzACWMaf14LbzbH4dgUpt2QZHBRScurn5Wx8bUYxFg11QCgmT7QjWYW3wHZFbLuA6xkFnAxKkg8
```

## Reference
[nft-drop-starter-project](https://github.com/buildspace/nft-drop-starter-project)
[build-space](https://buildspace.so/p/ship-solana-nft-collection)