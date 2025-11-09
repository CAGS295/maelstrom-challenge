use bloomfilter::Bloom;
use message::broadcast::Payload;
use message::broadcast::EXPECT_ELEMENTS;
use message::broadcast::FP_RATE;
use message::init::Init;
use message::Reply;
use serde::Serialize;
use std::collections::BTreeSet;
use std::io::stdin;
use std::io::BufRead;
use std::io::Write;

pub mod message;

use message::Body;
pub use message::Message;

#[derive(Debug, Serialize, Clone)]
pub enum Event<P> {
    Message(Message<P>),
    Sync,
}

pub struct Node {
    msg_id: u64,
    node_id: String,
    messages: BTreeSet<u64>,
    neighborhood: Vec<String>,
    index: Bloom<u64>,
}

impl Node {
    pub fn init<W: Write>(writer: W) -> Self {
        let stdin = stdin().lock();
        let line = stdin
            .lines()
            .next()
            .expect("Init msg always present")
            .unwrap();
        let msg: Message<Init> = serde_json::from_str(&line).unwrap();
        let mut node = Self {
            node_id: msg.body.payload.node_id.clone(),
            msg_id: 0,
            messages: BTreeSet::new(),
            neighborhood: vec![],
            index: Bloom::new_for_fp_rate(EXPECT_ELEMENTS, FP_RATE).unwrap(),
        };
        node.handle(Event::Message(msg), writer);
        node
    }

    pub fn handle<P, R>(&mut self, msg: Event<P>, mut writer: impl Write)
    where
        Message<P>: for<'a> Reply<R, &'a mut Self>,
        R: Serialize,
        P: Serialize,
    {
        match msg {
            Event::Message(msg) => {
                if let Some(res) = msg.response(self, self.msg_id) {
                    res.send(&mut writer);
                    self.msg_id += 1;
                };
            }
            Event::Sync => {
                for dest in &self.neighborhood {
                    Message {
                        src: self.node_id.clone(),
                        dest: dest.clone(),
                        body: Body {
                            msg_id: self.msg_id,
                            payload: Payload::Gossip {
                                bloom: self.index.clone(),
                            },
                        },
                    }
                    .send(&mut writer);
                    self.msg_id += 1;
                }
            }
        };
    }
}
