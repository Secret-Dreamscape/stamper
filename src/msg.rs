use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use secret_toolkit::utils::HandleCallback;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
  pub phonebook_addr: HumanAddr,
  pub phonebook_hash: String,
  pub nft_addr: HumanAddr,
  pub nft_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
  UpdateContracts {
    phonebook_addr: HumanAddr,
    phonebook_hash: String,
    nft_addr: HumanAddr,
    nft_hash: String,
  },
  Stamp {
    nft_id: String,
    word_id: u16,
    callee: HumanAddr,
  },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractNotQueriable {}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListContractsResponse {
  pub contracts: Vec<Contract>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Contract {
  pub address: HumanAddr,
  pub label: String,
  pub private: bool,
  pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PhonebookQueryMsg {
  GetList {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NFTContractHandlers {
  StampWord {
    token_id: String,
    word_id: u16,
    points: u16,
    callee: HumanAddr,
  },
}

impl HandleCallback for NFTContractHandlers {
  const BLOCK_SIZE: usize = 256;
}
