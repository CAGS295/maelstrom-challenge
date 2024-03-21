#!/usr/bin/env bash
set -ueo > /dev/null

case $1 in 
    echo)
    cargo build && ../maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 3
    ;;
    unique-id)
    cargo build && ../maelstrom/maelstrom test -w unique-ids --bin target/debug/unique_id --node-count 1 --time-limit 3
    ;;
    *)
    echo $1 not supported
    ;;
esac


