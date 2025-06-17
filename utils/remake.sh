#!/bin/bash

cd ../../arwing
cargo build
cd ../tm_replay_parser/utils
../../arwing/target/debug/rwing --game-path \
"/home/alex/Slippi/desync/gosu-7-21.slp" \
--frame 2515 --export 300 "rwing-export"
