#!/bin/bash

cd ../../arwing
cargo build
cd ../tm_replay_parser/utils
../../arwing/target/debug/rwing --game-path \
"/home/alex/Slippi/desyncs/f326.slp" \
--frame 326 --export 300 "rwing-export"
