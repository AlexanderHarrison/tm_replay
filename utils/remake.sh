#!/bin/bash

cd ../../arwing
cargo build
cd ../tm_replay_parser/utils
../../arwing/target/debug/rwing --game-path \
"/home/alex/Slippi/desync/gosu-7-19.slp" \
--frame 2669 --hmn-port low --export 300 "rwing-export"
