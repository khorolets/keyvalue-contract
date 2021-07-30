use near_sdk::near_bindgen;

use crate::*;

#[near_bindgen]
impl KeyValueContract {
    /// Returns the account ID of the owner
    pub fn get_owner_account_id(&self) -> Vec<AccountId> {
        vec![env::current_account_id(), self.get_master_account()]
    }

    /// Returns the values for the given keys
    pub fn get_keys(&self, keys: Vec<String>) -> HashMap<String, String> {
        let mut result = HashMap::<String, String>::new();
        for key in keys.into_iter() {
            if let Some(value) = self.store.get(&key) {
                result.insert(key, value);
            }
        }
        result
    }
}
