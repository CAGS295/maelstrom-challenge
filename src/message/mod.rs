use std::io::Write;

use serde::{Deserialize, Serialize};

pub mod broadcast;
pub mod echo;
pub mod init;
pub mod unique_id;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Message<Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<Payload>,
}

impl<P> Message<P> {
    pub fn response<R, S>(self, state: S, msg_id: u64) -> Option<Message<R>>
    where
        Message<P>: Reply<R, S>,
    {
        let src = self.dest.clone();
        let dest = self.src.clone();
        self.reply(state).map(|payload| Message {
            src,
            dest,
            body: Body { msg_id, payload },
        })
    }
}

impl<P: Serialize> Message<P> {
    pub fn send(self, writer: &mut impl Write) {
        serde_json::to_writer(&mut *writer, &self).unwrap();
        writer.write_all(b"\n").unwrap();
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Body<Payload> {
    pub msg_id: u64,
    #[serde(flatten)]
    pub payload: Payload,
}

pub trait Reply<R, S> {
    fn reply(self, state: S) -> Option<R>;
}
