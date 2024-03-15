use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;

use maelstrom_challenge::Node;

fn main() {
    Node::init(stdout().lock());
    eprintln!("pad");

    let stdin = stdin().lock();
    for line in stdin.lines() {
        let line = line.expect("read line");
        eprintln!("{line}");
    }
}
