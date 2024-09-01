use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;

use maelstrom_challenge::message::echo::Payload;
use maelstrom_challenge::Event;
use maelstrom_challenge::Message;
use maelstrom_challenge::Node;

fn main() {
    let mut node = Node::init(stdout().lock());

    let stdin = stdin().lock();
    for line in stdin.lines() {
        let msg: Message<Payload> = serde_json::from_str(&line.unwrap()).unwrap();
        node.handle::<Payload, _>(Event::Message(msg), &mut stdout().lock())
            .unwrap();
    }
}
