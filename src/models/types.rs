use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub name: String,
    pub street: String,
    pub city: String,
    pub state: String,
}
