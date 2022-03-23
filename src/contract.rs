use cosmwasm_std::{
  to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
  QueryRequest, StdError, StdResult, Storage, WasmQuery,
};
use secret_toolkit::utils::HandleCallback;

use crate::msg::{
  ContractNotQueriable, HandleMsg, InitMsg, ListContractsResponse, NFTContractHandlers,
  PhonebookQueryMsg, QueryMsg,
};
use crate::state::{config, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  msg: InitMsg,
) -> StdResult<InitResponse> {
  let state = State {
    owner:          env.message.sender.clone(),
    phonebook_addr: msg.phonebook_addr,
    phonebook_hash: msg.phonebook_hash,
    nft_addr:       msg.nft_addr,
    nft_hash:       msg.nft_hash,
  };

  config(&mut deps.storage).save(&state)?;

  Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  msg: HandleMsg,
) -> StdResult<HandleResponse> {
  match msg {
    HandleMsg::Stamp {
      nft_id,
      word_id,
      callee,
    } => try_stamp(deps, env, nft_id, word_id, callee),
    HandleMsg::UpdateContracts {
      phonebook_addr,
      phonebook_hash,
      nft_addr,
      nft_hash,
    } => try_update_contracts(
      deps,
      env,
      phonebook_addr,
      phonebook_hash,
      nft_addr,
      nft_hash,
    ),
  }
}

fn try_update_contracts<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  phonebook_addr: cosmwasm_std::HumanAddr,
  phonebook_hash: String,
  nft_addr: cosmwasm_std::HumanAddr,
  nft_hash: String,
) -> Result<HandleResponse, StdError> {
  let mut state = config(&mut deps.storage).load()?;

  if env.message.sender != state.owner {
    return Err(StdError::generic_err("Caller is not the owner".to_string()));
  }

  state.phonebook_addr = phonebook_addr;
  state.phonebook_hash = phonebook_hash;
  state.nft_addr = nft_addr;
  state.nft_hash = nft_hash;

  config(&mut deps.storage).save(&state)?;
  Ok(HandleResponse::default())
}

pub fn try_stamp<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  nft_id: String,
  word_id: u16,
  callee: HumanAddr,
) -> StdResult<HandleResponse> {
  let state = config(&mut deps.storage).load()?;
  let list_of_allowed_contracts_response =
    deps
      .querier
      .query::<ListContractsResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr:      state.phonebook_addr,
        callback_code_hash: state.phonebook_hash,
        msg:                to_binary(&PhonebookQueryMsg::GetList {})?,
      }))?;

  let list_of_allowed_contracts = list_of_allowed_contracts_response.contracts;
  let mut found = false;
  for contract in list_of_allowed_contracts {
    if contract.address == env.message.sender {
      found = true;
      break;
    }
  }
  if !found {
    return Err(StdError::generic_err(format!("Go away, cheater!")));
  }

  let nft_message = NFTContractHandlers::StampWord {
    token_id: nft_id,
    word_id,
    points: 1,
    callee,
  };

  let cosmos_msg =
    nft_message.to_cosmos_msg(state.nft_hash.clone(), state.nft_addr.clone(), None)?;

  Ok(HandleResponse {
    messages: vec![cosmos_msg],
    log:      vec![],
    data:     None,
  })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
  _deps: &Extern<S, A, Q>,
  _msg: QueryMsg,
) -> StdResult<Binary> {
  Ok(to_binary(&ContractNotQueriable {})?)
}
