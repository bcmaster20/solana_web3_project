# paulx solana escrow contract & clients

Reference implementation for the guide https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/

The contract is in [program](program) and the tests are in [scripts/src](scripts/src)

## Build Program
```
$ cd program && cargo build-bpf
$ solana program deploy ~/program/target/deploy/solana_escrow.so 
Program Id: H3QSPKqrbM4iNXacrFn92Uuma8a6cC8tVYvRVVa9RVDa
Copy the Program Id and Paste to scripts/keys/program_pub.json.
```

## Build Scripts
```
npm i
npm i --save-dev @types/bn.js
npm run setup-alice
npm run alice-bob
npm run all
```


