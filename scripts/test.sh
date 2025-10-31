#!/usr/bin/env bash
set -ueo > /dev/null

case $1 in 
    echo)
    cargo build && ./maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 2 --time-limit 3
    ;;
    unique-id)
    cargo build && ./maelstrom/maelstrom test -w unique-ids --bin target/debug/unique_id --node-count 2 --time-limit 3
    ;;
    broadcast)
    cargo build && ./maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 5 --time-limit 5 --rate 10
    ;;
    *)
    echo $1 not supported
    ;;
esac


