#!/usr/bin/env bash

cargo build && ../maelstrom/maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 3
