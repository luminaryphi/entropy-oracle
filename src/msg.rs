use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::utils::HandleCallback;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub adminseed: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Export {
        recipient_hash: String,
        entropy: String,
    },
    Collect {
        entropy: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum EntropyHandleMsg {
    ReceiveEntropy { entropy: [u8; 32] },
}

impl HandleCallback for EntropyHandleMsg {
    const BLOCK_SIZE: usize = 256;
}
