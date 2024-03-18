use serde::{Deserialize, Serialize};
pub mod init;

pub use init::Extra;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Message {
    src: String,
    dest: String,
    pub body: Body,
}

impl Message {
    pub fn reply(&self, body: Body) -> Self {
        Message {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Body {
    pub msg_id: u64,
    #[serde(flatten)]
    pub extra: init::Extra,
}
