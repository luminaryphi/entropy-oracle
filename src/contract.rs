use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply,
    Response, Storage, SubMsg, SubMsgResult,
};

use crate::error::ContractError;
use crate::msg::{EntropyHandleMsg, ExecuteMsg, InstantiateMsg};
use crate::state::{State, CONFIG_KEY};

use sha2::Digest;
use std::convert::TryInto;

use secret_toolkit::utils::HandleCallback;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    //Takes admin seed and turns into 32 bit array entropy
    let hashvalue = sha2::Sha256::digest(msg.adminseed.as_bytes());
    let hash: [u8; 32] = hashvalue.as_slice().try_into().expect("Wrong length");

    let state = State { seed: hash };

    //Save State
    CONFIG_KEY.save(deps.storage, &state)?;

    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let mut state: State = CONFIG_KEY.load(deps.storage)?;
    match msg {
        ExecuteMsg::Export {
            entropy,
            recipient_hash,
        } => export_entropy(deps, env, recipient_hash, entropy, &mut state, info),
        ExecuteMsg::Collect { entropy } => collect_entropy(deps, env, entropy, &mut state, info),
    }
}

//---------------------KEY FUNCTIONS------------------------------------------------------------------------------------------------

pub fn export_entropy(
    deps: DepsMut,
    env: Env,
    recipient_hash: String,
    entropy: String,
    state: &mut State,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    //Load state

    //Stored Entropy
    let new_data: String = format!(
        "{:?}+{}+{}+{}+{}",
        state.seed, entropy, &env.block.height, &env.block.time, &info.sender
    );

    let hashvalue = sha2::Sha256::digest(new_data.as_bytes());
    let hash: [u8; 32] = hashvalue.as_slice().try_into().expect("Wrong length");

    //Exported Entropy
    let export_data: String = format!("{:?}+{}", state.seed, entropy);

    let export_hashvalue = sha2::Sha256::digest(export_data.as_bytes());
    let export_hash: [u8; 32] = export_hashvalue
        .as_slice()
        .try_into()
        .expect("Wrong length");

    //Save new entropy
    state.seed = hash;

    CONFIG_KEY.save(deps.storage, &state)?;

    //Format and export entropy to sender
    let entropy_msg = EntropyHandleMsg::ReceiveEntropy {
        entropy: export_hash,
    };
    let cosmos_msg = entropy_msg.to_cosmos_msg(recipient_hash, info.sender.to_string(), None)?;

    let entropy_submsg = SubMsg::reply_on_error(cosmos_msg, 1);

    Ok(Response::new().add_submessage(entropy_submsg))
}

pub fn collect_entropy(
    deps: DepsMut,
    env: Env,
    entropy: String,
    state: &mut State,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    //Load state

    //Stored Entropy
    let new_data: String = format!(
        "{:?}+{}+{}+{}+{}",
        state.seed, entropy, &env.block.height, &env.block.time, &info.sender
    );

    let hashvalue = sha2::Sha256::digest(new_data.as_bytes());
    let hash: [u8; 32] = hashvalue.as_slice().try_into().expect("Wrong length");

    //Save new entropy
    state.seed = hash;

    CONFIG_KEY.save(deps.storage, &state)?;

    Ok(Response::new())
}