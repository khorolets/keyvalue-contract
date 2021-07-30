use crate::*;

/********************/
/* Internal methods */
/********************/

impl KeyValueContract {
    pub fn assert_owner(&self) {
        if &env::current_account_id() != &env::predecessor_account_id() {
            assert_eq!(
                &env::predecessor_account_id(),
                &self.get_master_account(),
                "Can only be called by the owner"
            )
        }
    }

    pub fn get_master_account(&self) -> AccountId {
        if let Some((_, master_account)) = env::current_account_id().split_once(".") {
            master_account.to_string()
        } else {
            env::current_account_id()
        }
    }
}
