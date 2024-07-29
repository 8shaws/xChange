use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailVerifyData {
    pub id: String,
    pub mail: String,
}
