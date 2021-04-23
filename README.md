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

# Create deployment account (optional)
near create-account counter.monkeyis.testnet --masterAccount=monkeyis.testnet  --initialBalance 10000000

# Deploy contract to account
# Unlike Ethereum, the contract gets deployed to the same account
near deploy --wasmFile target/wasm32-unknown-unknown/release/near_rust_tutorial.wasm --accountId counter.monkeyis.testnet

# Calling contract functions

# Use 'call' for functions that change state
near call monkeyis.testnet increment --accountId monkeyis.testnet
near call monkeyis.testnet decrement --accountId monkeyis.testnet

# Use 'view' for read-only functions
near view monkeyis.testnet get_num --accountId monkeyis.testnet
```

## Testing

Look at [src/lib.rs](./src/lib.rs) for boilerplate and test code structure. Run tests using `cargo test` command or by pressing run test buttons inside VS Code.

## Cross contract functionality

### 1. Deploying another contract

Instances of contract B will be deployed by contract A.

1. Write and compile contract B in a separate folder.
2. Copy the compiled `.wasm` file to contract A's directory
3. Source code to deploy contract from contract:

   ```rs
    pub fn deploy_contract(&self, account_id: String, amount: U128) {
        Promise::new(account_id)
            .create_account()
            .transfer(amount.0)
            .add_full_access_key(env::signer_account_pk())
            .deploy_contract(
                /* Path to compiled .wasm file of contract  */
                include_bytes!("./postbox_contract.wasm").to_vec(),
            );
    }
   ```

**Note**: A newly created account must be under a namespace of the creator account. Suppose contract A is deployed to `A.testnet` then B must be deployed to `B.A.testnet`. Otherwise `CreateAccountNotAllowed` will be thrown.

4. Call the `deploy_contract` function on deployed contract `counter.monkeyis.testnet`

```sh
# Deploy contract to address crossctr.counter.monkeyis.testnet with starting balance of 10 Near
near call counter.monkeyis.testnet deploy_contract '{ "account_id": "gg.counter.monkeyis.testnet", "amount": "10000000000000000000000000" }' --accountId counter.monkeyis.testnet
```

5. Querying the newly created contract

```sh
near view gg.counter.monkeyis.testnet get_message --accountId monkeyis.testnet
```

### 2. Transfer money to and between contracts

**Code**

```rs
#[payable] // Attribute is needed to accept payments. Methods are non-payable by default.
pub fn transfer_money(&mut self, account_id: String) {
   let deposit = env::attached_deposit(); // Read transferred amount

   // Contract to contract transfer
   Promise::new(account_id).transfer(
      deposit
   );
}
```

We are making two transfers here:
- **Transfer from user's account to contract**

   ```sh
   near call counter.monkeyis.testnet transfer_money '{ "account_id": "gg.counter.monkeyis.testnet" }' --accountId counter.monkeyis.testnet --amount 2
   ```

- **Contract to contract transfer**

### 3. Cross contract calls using high level API

Two types of calls are possible:
- **Update state** of callee contract
- **Read state** of callee contract: We can either
   - Just return the obtained value
   - Perform computation on this value and return the result.

### Updating state of callee contract

1. Create a trait with `#[ext_contract]` which contains method signatures of callee contract.

   ```rs
   #[ext_contract]
   pub trait Postbox {
      fn set_message(&mut self, message: u8);
      fn get_message(&self) -> u8;
   }
   ```

2. This generates a snake case namespace which lets us call the other contract.

   ```rs
   postbox_contract::set_message(
      message, // Parameters for set_message() function
      &account_id, // Postbox contract address
      0, // Amount to send
      env::prepaid_gas() / 2 // Gas
   );
   ```

3. Trigger this call using Near CLI

   ```sh
   near call counter.monkeyis.testnet set_postbox_message '{ "account_id": "gg.counter.monkeyis.testnet", "message": 5 }' --accountId counter.monkeyis.testnet
   ```

### Reading value from callee contract

1. To just read and return the value:

   ```rs
    pub fn get_postbox_message(&self, account_id: String) -> Promise {
        // Read from postbox contract and return value as result
        postbox::get_message(
            &account_id,
            0,
            env::prepaid_gas() / 2
        )
    }
   ```

   ```sh
   near call counter.monkeyis.testnet get_postbox_message '{ "account_id": "gg.counter.monkeyis.testnet" }' --accountId monkeyis.testnet
   ```

   **Note**: Return type must be `Promise`

2. To perform operations on the returned value, we need a callback to handle the promise.

   - Create a trait for the current contract, containing signature of the callback function.

   ```rs
   #[ext_contract]
   pub trait Counter {
      #[result_serializer(borsh)]
      fn postbox_callback(
         &self,
         #[callback]
         message: u8
      ) -> u8;
   }
   ```

   - `PromiseOrValue`, `then()` and `into()` are used to pass the results into the callback function

   ```rs
   pub fn process_postbox_message(&self, account_id: String) -> PromiseOrValue<u8> {
      let call_fee = env::prepaid_gas() / 4;

      postbox::get_message(
         &account_id,
         0,
         call_fee
      ).then(counter::postbox_callback(
         // then() automatically passes the results into current_account_id()
         // No need to explicitly specify the parameters
         &env::current_account_id(),
         0,
         call_fee
      )).into() // convert Promise into PromiseOrValue<u8>
   }
   ```

   - Create the callback function. Note use use of `#[callback]` attribute.

   ```rs
   #[private] // Should be callable from current contract
   pub fn postbox_callback(
      &self,
      #[callback] // Necessary for then() to pass results into this function
      message: u8
   ) -> u8 {
      log!("Got postbox message {:?}", message);

      message + 1
   }
   ```

#### TODO

- Is it possible to use the callback function without creating a trait for the contract? The contract implementation already contains the callback function signature.
- The docs use `#[result_serializer(borsh)]` and `#[serializer(borsh)]` to optimize the callback function. This was skipped due to number conversion issue when logging (0 -> 48), although correct value was returned. Near SDK falls back to JSON serializer by default. Find how to implement this. https://github.com/near/near-sdk-rs/blob/97fc632fcc58eb7ff7faad0be54ea8ec91dbf694/examples/cross-contract-high-level/src/lib.rs#L98

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