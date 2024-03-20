#!/usr/bin/env bash

case $1 in 
    echo)
    cargo build && ../maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 3
    ;;
esac


