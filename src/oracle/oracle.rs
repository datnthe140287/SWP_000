use near_sdk::{env, near_bindgen};
use std::convert::TryInto;

#[near_bindgen]
pub struct TokenOracle {}

impl TokenOracle {
    #[allow(dead_code)]
    pub fn get_token_price(&self) -> u128 {
        let token_contract_id = env::current_account_id();
        // Use the `promise_result` method to retrieve the result of a promise
        let promise_result = env::promise_result(0);
        match promise_result {
            // If the promise was successful, return the token balance of the contract divided by 100
            Ok(promise_result) => {
                let token_balance: u128 = promise_result.unwrap().as_u128();
                token_balance / 100
            }
            // If the promise failed, return an error value of 0
            Err(_) => 0,
        }
    }

    // Sends a promise to the Token contract to retrieve its total supply
    #[allow(dead_code)]
    pub fn request_token_balance(&self) {
        let token_contract_id = env::current_account_id();
        env::promise(
            token_contract_id.try_into().unwrap(),
            b"get_total_supply".to_vec(),
            0,
            0,
        );
    }
}
