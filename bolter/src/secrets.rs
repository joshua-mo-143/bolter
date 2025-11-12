use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize)]
#[serde(tag = "valueType", rename_all = "lowercase", content = "value")]
pub enum PlaintextOrSecret {
    Plaintext(String),
    Secret(String),
}

impl fmt::Debug for PlaintextOrSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plaintext(str) => {
                write!(f, "Plaintext value: {str}")
            }
            Self::Secret(_) => {
                write!(f, "Secret value: <redacted>")
            }
        }
    }
}
