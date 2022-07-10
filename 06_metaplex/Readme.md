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

# 22/07/10 Metaplex
## Source Download
```
1. HashLips Art Generation
https://github.com/HashLips/hashlips_art_engine/releases
(hashlips_art_engine-1.1.2_patch_v6.zip)

2. Candy-machine
https://github.com/exiled-apes/candy-machine-mint
(candy-machine-mint-main.zip)

3. Metaplex
https://github.com/metaplex-foundation/metaplex.git
(metaplex-master.zip)
```

## 1. HashLips Art
```
const network = NETWORK.sol;

// General metadata for solana
const namePrefix = "Meta Invest";
const description = "Some Meta Invest";
const baseUri = "ipfs://NewUriToReplace";

const solanaMetadata = {
  symbol: "META",
  seller_fee_basis_points: 1000, // Define how much % you want from secondary market sales 1000 = 10%
  external_url: "",
  creators: [
    {
      address: "5QuyZuwVs24h4tvQcsQnzV1iC9CmbaEz1A3BwHnV4fJy",
      share: 100,
    },
  ],
};

```

- Command
```
yarn generate (0-9)
yarn pixelate
yarn preview
yarn preview_gif
yarn rarity
mkdir build/assets
** cp images/*.png json/*.json =>  build/assets
```

## 2 Metaplex
### 2-1 Metaplex Upload assets
- New Solana Wallet
```
solana-keygen new --outfile ~/.config/solana/devnet.json

  BIP39 Passphrase (empty for none): 

  Wrote new keypair to ~/.config/solana/devnet.json
  ====================================================================================
  pubkey: 9hgfgFN1HtuCiTDwzZcv6m4dsxWsEueNCLxPFGtUHr7Q
  ====================================================================================
  Save this seed phrase and your BIP39 passphrase to recover your new keypair:
  warfare fame forum smooth carry quarter citizen mercy caution remain resemble action
  ====================================================================================

solana config set --keypair ~/.config/solana/devnet.json

```

- Metamask Install
```
cd metaplex-master/js
yarn install
yarn build
yarn bootstrap
```

- metaplex-master/config.json
```
{
    "price": 0.1,
    "number": 10,
    "gatekeeper": null,
    "solTreasuryAccount": "9hgfgFN1HtuCiTDwzZcv6m4dsxWsEueNCLxPFGtUHr7Q",
    "splTokenAccount": null,
    "splToken": null,
    "goLiveDate": "25 Dec 2021 00:00:00 GMT",
    "endSettings": null,
    "whitelistMintSettings": null,
    "hiddenSettings": null,
    "storage": "arweave-sol",
    "ipfsInfuraProjectId": null,
    "ipfsInfuraSecret": null,
    "nftStorageKey": null,
    "awsS3Bucket": null,
    "noRetainAuthority": false,
    "noMutable": false
}
```

- cany-machine asset verify
```
cd metaplex-master
npx ts-node js/packages/cli/src/candy-machine-v2-cli.ts verify_assets ./assets
  started at: 1657447479111
  Verifying token metadata for 10 (img+json) pairs
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/0.json
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/1.json
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/2.json
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/3.json
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/4.json
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/5.json
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/6.json
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/7.json
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/8.json
  Checking manifest file: /home/jaylee/01_project/solana_web3_project/08_metaplex_mint/metaplex-master/assets/9.json
  ended at: Sun Jul 10 2022 19:04:39 GMT+0900 (Japan Standard Time). time taken: 00:00:00
```
- cany-machine asset upload
```
npx ts-node js/packages/cli/src/candy-machine-v2-cli.ts upload -e devnet -k ~/.config/solana/devnet.json -cp config.json -c example ./assets
    wallet public key: 9hgfgFN1HtuCiTDwzZcv6m4dsxWsEueNCLxPFGtUHr7Q
    Using cluster devnet
    WARNING: The "arweave" storage option will be going away soon. Please migrate to arweave-bundle or arweave-sol for mainnet.

    Beginning the upload for 10 (img+json) pairs
    started at: 1657449479076
    initializing candy machine
    Candy machine address:  J3yKEFu8XNyQRTSMwVg5MU1xp34CHSCNu7fvUK3rtAdv
    Collection metadata address:  8DHAPtWrapdBLTiM3hmv1bgzSU13iA5VpjBoBmzKLXc7
    Collection metadata authority:  9hgfgFN1HtuCiTDwzZcv6m4dsxWsEueNCLxPFGtUHr7Q
    Collection master edition address:  HMXGjkTLDGZN6xVp27YCVxBGHxfPsGR5pGzp96sRmG48
    Collection mint address:  GV9XYWuTcVboerWLPVPvkHJEostirPvTtb8EFeB7wV82
    Collection PDA address:  CRL11m9cbsPfnvmHTjvVAaCusxSTPg59RTZBbbzGoBdR
    Collection authority record address:  Cy82YgydpwfzBv8zMEFgvTG67dcnQLUAq86ioruf8zy8
    Collection:  {
      collectionMetadata: '8DHAPtWrapdBLTiM3hmv1bgzSU13iA5VpjBoBmzKLXc7',
      collectionPDA: 'CRL11m9cbsPfnvmHTjvVAaCusxSTPg59RTZBbbzGoBdR',
      txId: '3ax3vLB7ktQLuyznPXtenoo8krphzgDX8wbN9nCZLjGcY3gstgqscP9LBCqBGx1rgtT1KqULgunmVyLFGGwbsUZ9'
    }
    initialized config for a candy machine with publickey: J3yKEFu8XNyQRTSMwVg5MU1xp34CHSCNu7fvUK3rtAdv
    [0] out of [10] items have been uploaded
    Starting upload for [10] items, format {"mediaExt":".png","index":"0"}
    Progress: [████████████████████████████████████████] 100% | 10/10
    Writing all indices in 1 transactions...
    Progress: [████████████████████████████████████████] 100% | 1/1
    Done. Successful = true.
    ended at: 2022-07-10T10:38:49.714Z. time taken: 00:00:50
```

- cany-machine asset upload
```
npx ts-node js/packages/cli/src/candy-machine-v2-cli.ts verify_upload -e devnet -k ~/.config/solana/devnet.json -c example
    wallet public key: 9hgfgFN1HtuCiTDwzZcv6m4dsxWsEueNCLxPFGtUHr7Q
    Using cluster devnet
    Checking 10 items that have yet to be checked...
    Looking at key  0
    Looking at key  1
    Looking at key  2
    Looking at key  3
    Looking at key  4
    Looking at key  5
    Looking at key  6
    Looking at key  7
    Looking at key  8
    Looking at key  9
    uploaded (10) out of (10)
    ready to deploy!

```
### 2-2 Candimachine Mint Web Site
```
cd candy-machine-mint-main

```

https://www.npmjs.com/package/ts-node#installation


## Reference
- [hashlips_art_engine](https://github.com/HashLips/hashlips_art_engine)
- [Mint-YT](https://www.youtube.com/watch?v=yk6cPxvvgTg&list=PLRtVkF1AnQ5icooBablJZUqVeBlUgnX5G&index=2)