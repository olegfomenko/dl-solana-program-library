# Upgrade program

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/upgrade-program)](https://crates.io/crates/upgrade-program)
[![Docs.rs](https://docs.rs/upgrade-program/badge.svg)](https://docs.rs/upgrade-program)


## Build

```shell
npm run build:utxo-base-program
```

## Deploy
```shell
solana program deploy --program-id ./dist/program/upgrade-keypair.json ./dist/program/utxo-base-program.so
```
