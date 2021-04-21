use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; // For IO serialization and deserialization
use near_sdk::{
     env, // Like context, provides info about caller
     near_bindgen
};

// Boilerplate for memory management
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

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
}

/*
* Functions outside impl act as private functions.
* They can't be invoked directly, but can be called by an invoked function
*/
fn check_overflow() {
    env::log(b"Ensure you don't overflow in production"); // alternative to .as_bytes()
    // env::log("Ensure you don't overflow in production".as_bytes());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
