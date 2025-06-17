#!/bin/bash

./remake.sh

export GOOD_GCI=$1
export BAD_GCI="rwing-export.gci"

./write_recsave "$GOOD_GCI"
./write_recsave "$BAD_GCI"

# only search within matchinit & savestate
./fbin "$GOOD_GCI.recsave" "$BAD_GCI.recsave" 0 54928 /home/alex/melee/tutor/tm_replay_parser/utils/run.sh

rm rwing-export*
