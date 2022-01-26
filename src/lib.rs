use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::store::{LookupMap, UnorderedMap};
use near_sdk::BorshStorageKey;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    List,
    ListPerAccount { account_id: AccountId },
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct VIPList {
    owner_id: AccountId,
    lists: LookupMap<AccountId, UnorderedMap<AccountId, u128>>,
}

#[near_bindgen]
impl VIPList {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            lists: LookupMap::new(StorageKey::List),
        }
    }

    pub fn add_list(&mut self, account_id: AccountId) {
        if !self.lists.contains_key(&account_id) {
            self.lists.insert(
                account_id.clone(),
                UnorderedMap::new(StorageKey::ListPerAccount { account_id }),
            );
        }
    }

    pub fn add_to_list(&mut self, account_id: AccountId, allowance: U128) {
        self.get_current_list_mut()
            .map(|list| list.insert(account_id, allowance.into()));
    }

    pub fn vip_allowance(&self, list_id: Option<AccountId>, account_id: AccountId) -> Option<U128> {
        let list_id = list_id.unwrap_or_else(|| env::predecessor_account_id());
        self.lists
            .get(&list_id)
            .and_then(|list| list.get(&account_id).map(|allowance| (*allowance).into()))
    }

    fn get_current_list_mut(&mut self) -> Option<&mut UnorderedMap<AccountId, u128>> {
        let caller = env::predecessor_account_id();
        self.lists.get_mut(&caller)
    }
}
