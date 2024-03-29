use message::init::Init;
use message::Reply;
use serde::Serialize;
use std::io::stdin;
use std::io::BufRead;
use std::io::Write;

pub mod message;

use message::Body;
pub use message::Message;

pub struct Node {
    msg_id: u64,
    node_id: String,
    messages: Vec<u64>,
}

impl Node {
    pub fn init<W: Write>(writer: W) -> Self {
        let stdin = stdin().lock();
        let line = stdin
            .lines()
            .into_iter()
            .next()
            .expect("Init msg always present")
            .unwrap();
        let msg: Message<Init> = serde_json::from_str(&line).unwrap();
        let mut node = Self {
            node_id: msg.body.payload.node_id.clone(),
            msg_id: 0,
            messages: vec![],
        };
        node.handle(msg, writer).unwrap();
        node
    }

    pub fn handle<P, R>(&mut self, msg: Message<P>, mut writer: impl Write) -> Result<(), ()>
    where
        Body<P>: for<'a> Reply<R, &'a mut Self>,
        R: Serialize,
    {
        let response = msg.response(Body {
            msg_id: self.msg_id,
            payload: msg.body.reply(self),
        });

        serde_json::to_writer(&mut writer, &response).unwrap();
        writer.write_all(b"\n").unwrap();

        self.msg_id += 1;

        Ok(())
    }
}
