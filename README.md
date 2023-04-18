# Cortado Finance

This is the repository for all the Cosmwasm contracts for Cortado Finance. Cortado is the premier, chain agnostic, options exchange with a built in on-chain market maker utilizing Option Vaults.

Utilizing a specially designed on-chain CLOB (central-limit order book) with built in on-chain market making through Option Vaults, Cortado is designed to be the most liquid and efficient crypto options exchange on the planet. By allowing liquidity providers to pool into option vaults, Cortado allows option creators to write options by creating derivatives on the option vaults.

Using a chain agnostic approach and hub and spoke model with Axelar and IBC, Cortado is chain agnostic meaning markets with underlyings from 70+ IBC chains and 12 EVM chains (including the top ones) can be created and then auctioned off on any DEX that Cortado launches its contracts on that supports Cosmwasm.

Interested in learning more? Our socials and Discord are coming soon!

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via: 

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw20_cortado.wasm .
ls -l cw20_cortado.wasm
sha256sum cw20_cortado.wasm
```

Or for a production-ready (optimized) build, run a build command in the
the repository root: https://github.com/CosmWasm/cw-plus#compiling.
