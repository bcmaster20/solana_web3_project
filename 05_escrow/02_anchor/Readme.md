# Anchor Escrow

## Project Init
```
$ anchor init anchor-escrow
```

### Programs/anchor-escrow/Cargo.toml
```
anchor-lang = "0.20.1"
anchor-spl = {version = "0.20.1"}
spl-token = {version = "3.3.0", features = ["no-entrypoint"]
```

### Test
```
npm install --save @solana/spl-token
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