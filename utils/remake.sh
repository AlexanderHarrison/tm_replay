#!/bin/bash

cd ../../arwing
./build build
cd ../tm_replay_parser/utils
export F=4620
../../arwing/target/debug/rwing --game-path \
"/home/alex/Slippi/desync/f$F.slp" \
--frame $F --hmn-port low --export 600 "rwing-export"
