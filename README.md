Status Message
==============

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/near-examples/rust-status-message)

<!-- MAGIC COMMENT: DO NOT DELETE! Everything above this line is hidden on NEAR Examples page -->

This smart contract saves and records the status messages of NEAR accounts that call it.

Windows users: please visit the [Windows-specific README file](README-Windows.md).

## Prerequisites
Ensure `near-cli` is installed by running:

```
near --version
```

If needed, install `near-cli`:

```
npm install near-cli -g
```

Ensure `Rust` is installed by running:

```
rustc --version
```

If needed, install `Rust`:

```
curl https://sh.rustup.rs -sSf | sh
```

Install dependencies

```
npm install
```

```bash
rustup target add wasm32-unknown-unknown
```

```
near login
```

Deploy the contract to your NEAR account:


Build the frontend:

```bash

```



```bash

```


```bash

```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:
```bash
cargo build --target wasm32-unknown-unknown --release
near deploy --wasmFile target/wasm32-unknown-unknown/release/token.wasm --accountId datng-token.testnet
near call datng-token.testnet new '{"name": "Luna", "symbol": "LUNA","decimals" : 18, "total_supply" : 1000}' --accountId datng-token.testnet
near view datng-token.testnet balance_of '{"account_id": "datng-token.testnet"}'




cargo build --target wasm32-unknown-unknown --release
near deploy --wasmFile target/wasm32-unknown-unknown/release/basis.wasm --accountId datng-basis-token.testnet
near call datng-basis-token.testnet new '{"name": "TerraUSD", "symbol": "UST","decimals" : 18, "total_supply" : 1000}' --accountId datng-basis-token.testnet




## Testing
To test run:
```bash
cargo test --package status-message -- --nocapture
```
