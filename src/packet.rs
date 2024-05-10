use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinPacket {
    pub name: String,
}
