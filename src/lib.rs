use near_sdk::{AccountId, BorshStorageKey, env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector};
use near_sdk::json_types::{U64};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    StoreStorageKey
}

#[derive(Clone, Debug, Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
pub struct MessageArgs {
    pub from_message_account: AccountId,
    pub to_message_account: AccountId,
    pub date_created: U64,
    pub body: String
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Store {
    pub conversations: Vector<MessageArgs>
}


impl Default for Store {
    fn default() -> Self {
        env::panic_str("NOT_INITIALIZED_YET");
    }
}

#[near_bindgen]
impl Store {
    #[init(ignore_state)]
    pub fn init() -> Self {
        assert!(!env::state_exists());
        Self {
            conversations: Vector::new(StorageKeys::StoreStorageKey)
        }
    }

    #[payable]
    pub fn new_message(
        &mut self,
        from: AccountId,
        to: AccountId,
        body: String,
    ) {
        env::log_str(format!("NEARCON_2022::new_message() Sender: {} Receiver: {}  body: {}",
                             env::predecessor_account_id().as_str(),
                             to.clone().as_str(),
                             body.clone().as_str())
            .as_str());

        let message_args = MessageArgs {
            from_message_account: from,
            to_message_account: to,
            date_created: U64(env::block_timestamp()),
            body
        };
        self.conversations.push(&message_args)
    }


    pub fn get_messages(
        &self,
    ) -> Vec<MessageArgs> {
        self.conversations.to_vec()
    }
}
