#!/bin/bash

# The std lib is missing so we have to specify the path.
ddlog -i graph.dl -L ~/differential-datalog/lib &&
(cd graph_ddlog && cargo build --release && target/release/graph_cli < ../graph.dat) 
# (cd graph_lib && cargo run --release -- 200 300 debug)