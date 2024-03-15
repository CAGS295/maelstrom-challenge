use maelstrom_challenge::Body;
use maelstrom_challenge::Extra;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;
use std::io::Write;

use maelstrom_challenge::Message;

struct Node {}

impl Node {
    //TODO generic reader writer;
    fn init<W: Write>(mut writer: W) {
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

fn main() {
    Node::init(stdout().lock());
    eprintln!("pad");

    let stdin = stdin().lock();
    for line in stdin.lines() {
        let line = line.expect("read line");
        eprintln!("{line}");
    }
}
