# Near Rust tutorial

## Directory structure

Setup rust environment using `cargo init --lib`

```
.
├── Cargo.toml
└── src
   └── lib.rs
```

- **Cargo.toml**: Similar to `package.json` in Node. It contains dependency list, build settings and package metadata.
- **lib.rs**: Contains contract.

## Contract structure

Look at [src/lib.rs](./src/lib.rs) for notes.

## Commands
```sh
# Compile
env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

# Deploy contract to account
# Unlike Ethereum, the contract gets deployed to the same account
near deploy --wasmFile target/wasm32-unknown-unknown/release/near_rust_tutorial.wasm --accountId monkeyis.testnet

# Calling contract functions

# Use 'call' for functions that change state
near call monkeyis.testnet increment --accountId monkeyis.testnet
near call monkeyis.testnet decrement --accountId monkeyis.testnet

# Use 'view' for read-only functions
near view monkeyis.testnet get_num --accountId monkeyis.testnet
```

## Rust notes

### Rust binary vs library
1. Binaries are standalone, created using `cargo init`. They have a **main.rs**.
2. Libraries: Used by other programs. Create using `cargo init --lib`. They have a **lib.rs** file.

### Installing dependencies
Use either:
1. `cargo install ABC`: For dependencies having binaries
2. Add dependency name and version to `[dependencies]` section of **Cargo.toml**. For Near SDK add `near-sdk = "3.1.0"`


### Rust attributes

Eg. `#[near_bindgen]`

An attribute is metadata applied to an item. It can be used for:
- Conditional compilation
- Set crate name, version etc
- Disabling lints
- Linking to a foreign library
- Marking functions as unit tests

### Syntax
```rs
#[attribute(value)]
```