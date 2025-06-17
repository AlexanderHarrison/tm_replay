#!/bin/bash

cd ../../arwing
cargo build
cd ../tm_replay_parser/utils
../../arwing/target/debug/rwing --game-path \
"/home/alex/Slippi/desync/f10210.slp" \
--frame 10200 --export 300 "rwing-export"
