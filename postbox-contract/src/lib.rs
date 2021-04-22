use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}, json_types::U128}; // For IO serialization and deserialization
use near_sdk::{
     env, // Like context, provides info about caller
     near_bindgen,

     // Cross contract calls
     Promise
};

// Boilerplate for memory management
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

// Near contracts have a struct and impl pattern

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Postbox {
    message: u8
}

// Contains functions which will manipulate the struct data
#[near_bindgen]
impl Postbox {
    pub fn get_message(&self) -> u8 {
        env::log("Returning count".as_bytes());
        return self.message;
    }

    pub fn set_message(&mut self, message: u8) {
        let log_msg = format!("Setting message {}", message);
        env::log(log_msg.as_bytes());
        self.message = message;
    }
}
