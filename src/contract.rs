use cosmwasm_std::{Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage};

use crate::msg::{EntropyHandleMsg, HandleMsg, InitMsg};
use crate::state::{load, save, State};

use sha2::{Digest};
use std::convert::TryInto;

use secret_toolkit::utils::{HandleCallback};


pub const STATE_KEY: &[u8] = b"state";



pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {


    //Takes admin seed and turns into 32 bit array entropy
    let hashvalue = sha2::Sha256::digest(msg.adminseed.as_bytes());
    let hash: [u8; 32] = hashvalue.as_slice().try_into().expect("Wrong length");
    
    

    let state = State {

        seed: hash,

    };


    //Save State
    save(&mut deps.storage, STATE_KEY, &state)?;


    Ok(InitResponse::default())
}





//HANDLE LIST
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Entropy {entropy , recipient_hash } => gather_entropy(deps, env, recipient_hash, entropy),
    }
}



//---------------------KEY FUNCTIONS------------------------------------------------------------------------------------------------






pub fn gather_entropy<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    recipient_hash: String,
    entropy: String
) -> StdResult<HandleResponse> {


    //Load state
    let mut state: State = load(&mut deps.storage, STATE_KEY)?;

    //Stored Entropy
    let new_data: String = format!("{:?}+{}+{}+{}+{}", state.seed, entropy, &env.block.height, &env.block.time, &env.message.sender);

    let hashvalue = sha2::Sha256::digest(new_data.as_bytes());
    let hash: [u8; 32] = hashvalue.as_slice().try_into().expect("Wrong length");

    //Exported Entropy
    let export_data: String = format!("{:?}+{}", state.seed, entropy);

    let export_hashvalue = sha2::Sha256::digest(export_data.as_bytes());
    let export_hash: [u8; 32] = export_hashvalue.as_slice().try_into().expect("Wrong length");



    //Save new entropy
    state.seed = hash;

    save(&mut deps.storage, STATE_KEY, &state)?;


    //Format and export entropy to sender
    let entropy_msg = EntropyHandleMsg::ReceiveEntropy {
        entropy: export_hash,
    };

    let cosmos_msg = entropy_msg.to_cosmos_msg(
        recipient_hash,
        env.message.sender,
        None,
    )?;



    Ok(HandleResponse {
        messages: vec![cosmos_msg],
        log: vec![],
        data: None,
    })


}
