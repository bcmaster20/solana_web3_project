# Anchor Escrow

## Project Init
```
$ anchor init anchor-escrow
```

### Programs/anchor-escrow/Cargo.toml
```
anchor-spl = {version = "0.24.2"}
spl-token = {version = "3.3.0", features = ["no-entrypoint"]
```

### Test
```
yarn add @solana/spl-token
yarn add @solana/web3.js
```
### Test Fixed.
Add below part in package.json
```
"resolutions": {
  "@solana/web3.js": "1.36.0"
}  
```

## Reference
- [Anchor Escrow](https://book.solmeet.dev/notes/intro-to-anchor)