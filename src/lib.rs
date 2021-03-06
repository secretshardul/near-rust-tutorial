use near_sdk::{PromiseOrValue, borsh::{ self, BorshDeserialize, BorshSerialize }}; // For IO serialization and deserialization
use near_sdk::{
     env, // Like context, provides info about caller
     log,
     near_bindgen,
     json_types::U128,

     // Cross contract calls
     Promise,
     ext_contract
};

// Boilerplate for memory management
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

/* Calling external contracts */

#[ext_contract]
pub trait Postbox {
    fn set_message(&mut self, message: u8);
    fn get_message(&self) -> u8;
}

#[ext_contract]
pub trait Counter {
    fn postbox_callback(
        &self,
        #[callback]
        message: u8
    ) -> u8;
}

// Near contracts have a struct and impl pattern

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: u8
}

// Contains functions which will manipulate the struct data
#[near_bindgen]
impl Counter {
    pub fn get_num(&self) -> u8 {
        env::log("Returning count".as_bytes());
        return self.val;
    }

    pub fn increment(&mut self) {
        check_overflow();
        self.val += 1;
        let log_message = format!("Incremented to {}", self.val);
        env::log(log_message.as_bytes());
    }

    pub fn decrement(&mut self) {
        self.val -= 1;
        let log_message = format!("Decremented to {}", self.val);
        env::log(log_message.as_bytes());
    }

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

    #[payable] // Attribute is needed to accept payments. Methods are non-payable by default.
    pub fn transfer_money(&mut self, account_id: String) {
        let deposit = env::attached_deposit(); // Read transferred amount

        let log_message = format!("Attached deposit {}", deposit);
        env::log(log_message.as_bytes());

        // Contract to contract transfer
        Promise::new(account_id).transfer(
            deposit
        );
    }

    /* Cross contract calls using high level API */

    /**
    * Call Postbox contract and update it's state
    */
    pub fn set_postbox_message(&mut self, account_id: String, message: u8) {
        postbox::set_message(
            message, // Parameters for set_message() function
            &account_id, // Postbox contract address
            0, // Amount to send
            env::prepaid_gas() / 2 // Gas
        );
    }

    /**
    * Call Postbox contract and read it's state. The read value is returned as a result.
    */
    pub fn get_postbox_message(&self, account_id: String) -> Promise {
        // Read from postbox contract and return value as result
        postbox::get_message(
            &account_id,
            0,
            env::prepaid_gas() / 2
        )
    }

    #[private] // Should be callable from current contract
    pub fn postbox_callback(
        &self,
        #[callback] // Necessary for then() to pass results into this function
        message: u8
    ) -> u8 {
        log!("Got postbox message {:?}", message);

        message + 1
    }

    // TODO optimize with BORSH serializer
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
        )).into()
    }
}

/*
* Functions outside impl act as private functions.
* They can't be invoked directly, but can be called by an invoked function
*/
fn check_overflow() {
    env::log(b"Ensure you don't overflow in production"); // alternative to .as_bytes()
}

#[cfg(test)]
mod tests {
    // Testing boilerplate
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // Context initializer function
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // Test cases here
    #[test]
    fn increment() {
        // Initialize context
        let context = get_context(vec![], false);
        testing_env!(context);

        // Operate on contract data
        let mut contract = Counter { val: 0 };
        contract.increment();

        // Compare with expected output
        assert_eq!(1, contract.val);
    }

    #[test]
    fn decrement() {
        // Initialize context
        let context = get_context(vec![], false);
        testing_env!(context);

        // Operate on contract data
        let mut contract = Counter { val: 1 };
        contract.decrement();

        // Compare with expected output
        assert_eq!(0, contract.val);
    }
}
