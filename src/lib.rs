use serde::{Deserialize, Serialize};
use std::io::stdin;
use std::io::BufRead;
use std::io::Write;

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
    pub extra: Extra,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Extra {
    Echo {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: u64,
    },
}

pub struct Node {}

impl Node {
    //TODO generic reader writer;
    pub fn init<W: Write>(mut writer: W) {
        let stdin = stdin().lock();
        let line = stdin
            .lines()
            .into_iter()
            .next()
            .expect("Init msg always present")
            .unwrap();
        let msg: Message = serde_json::from_str(&line).unwrap();
        serde_json::to_writer(
            &mut writer,
            &msg.reply(Body {
                msg_id: 0,
                extra: Extra::InitOk {
                    in_reply_to: msg.body.msg_id,
                },
            }),
        )
        .unwrap();
        writer.write_all(b"\n").unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn echo_format() {
        let obj = Body {
            msg_id: 0,
            extra: Extra::Echo {
                echo: String::from_str("hola").unwrap(),
            },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":0,"type":"echo","echo":"hola"}"#
        );
    }

    #[test]
    fn init_format() {
        let obj = Body {
            msg_id: 0,
            extra: Extra::Init {
                node_id: String::from_str("n0").unwrap(),
                node_ids: vec![String::from_str("n0").unwrap()],
            },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":0,"type":"init","node_id":"n0","node_ids":["n0"]}"#
        );
    }

    #[test]
    fn init_ok_format() {
        let obj = Body {
            msg_id: 0,
            extra: Extra::InitOk { in_reply_to: 0 },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":0,"type":"init_ok","in_reply_to":0}"#
        );
    }
}
