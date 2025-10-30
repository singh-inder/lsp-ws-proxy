use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// Request ID
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Id {
    /// Numeric ID.
    Number(u64),
    /// String ID.
    String(String),
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Number(id) => Display::fmt(id, f),
            Self::String(id) => fmt::Debug::fmt(id, f),
        }
    }
}

/// Unknown message type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unknown(serde_json::Value);
