# Cortado Finance

This is the repository for all the Cosmwasm contracts for Cortado Finance. Cortado is the premier, chain agnostic, options exchange with a built in on-chain market maker.

Utilizing a highly specially designed on-chain orderbook with built in on-chain market making, Cortado is designed to be the most liquid and efficient crypto options exchange on the planet. Our goal is to become a better version of CBOE for crypto but entirely decentralized and trustless.

Built on top of Archway with Axelar and IBC built in, Cortado is chain agnostic meaning markets with underlyings from 60+ IBC chains and 7 EVM chains (including the top ones) can be created making Cortado the most chain agnostic options exchange. Our end goal is to support every single crypto asset on the planet.

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
