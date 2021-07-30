//! A smart contract that allows tokens to be locked up.

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId};
use std::collections::HashMap;

pub use crate::getters::*;
pub use crate::internal::*;
pub use crate::owner::*;

pub mod getters;
pub mod internal;
pub mod owner;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct KeyValueContract {
    pub store: UnorderedMap<String, String>,
}

impl Default for KeyValueContract {
    fn default() -> Self {
        Self {
            store: UnorderedMap::<String, String>::new(b"s"),
        }
    }
}

#[near_bindgen]
impl KeyValueContract {
    #[init]
    pub fn new(initial_store: Option<HashMap<String, String>>) -> Self {
        let mut store = UnorderedMap::<String, String>::new(b"s");

        if let Some(initial_store) = initial_store {
            store.extend(initial_store.into_iter());
        }

        Self { store }
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(test)]
// mod tests {
//     use std::convert::TryInto;

//     use near_sdk::{testing_env, MockedBlockchain, PromiseResult, VMContext};

//     use test_utils::*;

//     use super::*;

//     mod test_utils;

//     pub type AccountId = String;

//     const SALT: [u8; 3] = [1, 2, 3];

//     fn basic_context() -> VMContext {
//         get_context(
//             system_account(),
//             to_yocto(LOCKUP_NEAR),
//             0,
//             to_ts(GENESIS_TIME_IN_DAYS),
//             false,
//         )
//     }

//     testing_env!(context.clone());
// }
