use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
  pub owner: HumanAddr,
  pub phonebook_addr: HumanAddr,
  pub phonebook_hash: String,
  pub nft_addr: HumanAddr,
  pub nft_hash: String,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
  singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
  singleton_read(storage, CONFIG_KEY)
}
