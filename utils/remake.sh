#!/bin/bash

cd ../arwing
cargo build
cd ../tm_replay_parser
../arwing/target/debug/rwing --game-path \
"/home/alex/Downloads/desyncs/marth-2867.slp" \
--frame 2867 --export 300 "rwing-export"
