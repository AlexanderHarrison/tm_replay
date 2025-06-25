#!/bin/bash

cd ../../arwing
cargo build
cd ../tm_replay_parser/utils
../../arwing/target/debug/rwing --game-path \
"/home/alex/Slippi/desync/f9143.slp" \
--frame 9143 --hmn-port low --export 300 "rwing-export"
