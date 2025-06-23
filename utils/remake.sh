#!/bin/bash

cd ../../arwing
cargo build
cd ../tm_replay_parser/utils
../../arwing/target/debug/rwing --game-path \
"/home/alex/Downloads/desyncs/1571.slp" \
--frame 1571 --hmn-port low --export 300 "rwing-export"
