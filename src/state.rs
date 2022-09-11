use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cosmwasm_std::{StdError, StdResult, Storage};

use std::any::type_name;

use secret_toolkit::serialization::{Bincode2, Serde};
use secret_toolkit::storage::Item;

pub static CONFIG_KEY: Item<State> = Item::new(b"config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub seed: [u8; 32],
}
