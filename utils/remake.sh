#!/bin/bash

cd ../../arwing
cargo build
cd ../tm_replay_parser/utils
../../arwing/target/debug/rwing --game-path \
"/home/alex/Downloads/desyncs/f5220.slp" \
--frame 5580 --hmn-port low --export 300 "rwing-export"
