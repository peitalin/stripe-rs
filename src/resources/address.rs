use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}
