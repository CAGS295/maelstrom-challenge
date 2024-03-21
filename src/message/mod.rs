use serde::{Deserialize, Serialize};
pub mod echo;
pub mod init;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Message<Payload> {
    src: String,
    dest: String,
    pub body: Body<Payload>,
}

impl<Payload> Message<Payload> {
    pub fn response<R>(&self, body: Body<R>) -> Message<R> {
        Message {
            src: self.dest.clone(),
            dest: self.src.clone(),
            body,
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Body<Payload> {
    pub msg_id: u64,
    #[serde(flatten)]
    pub payload: Payload,
}

pub trait Reply<R, S> {
    fn reply(&self, state: S) -> R;
}
