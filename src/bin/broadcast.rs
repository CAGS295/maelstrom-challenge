use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

use maelstrom_challenge::message::broadcast::Payload;
use maelstrom_challenge::Event;
use maelstrom_challenge::Message;
use maelstrom_challenge::Node;

fn main() {
    let mut node = Node::init(stdout().lock());

    let (tx, rx) = channel();

    let tx_1 = tx.clone();
    spawn(move || {
        let stdin = stdin().lock();
        for line in stdin.lines() {
            let msg: Message<Payload> = serde_json::from_str(&line.unwrap()).unwrap();
            tx_1.send(Event::Message(msg)).unwrap();
        }
    });

    spawn(move || {
        let tx = tx;
        loop {
            sleep(Duration::from_millis(200));
            tx.send(Event::Sync).unwrap();
        }
    });

    while let Ok(event) = rx.recv() {
        node.handle::<Payload, _>(event, &mut stdout().lock());
    }
}
