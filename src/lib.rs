use std::io::stdin;
use std::io::BufRead;
use std::io::Write;

pub mod message;

use message::{Body, Extra, Message};

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
