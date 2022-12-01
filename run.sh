#!/bin/bash
cargo build
start=`date +%s.%N`
cargo run
end=`date +%s.%N`

runtime=$( echo "$end - $start" | bc -l )
echo -e "Execution time: ${runtime}"
