#!/bin/bash

# Build formula2ddlog runtime from formula2ddlog.dl

# Use `formula2ddlog` to transform graph.4ml into graph.dl
formula2ddlog_lib/target/release/formula2ddlog_lib examples/graph/graph.4ml > examples/graph/graph.dl &&
# The std lib is missing so we have to specify the path.
ddlog -i examples/graph/graph.dl -L ~/differential-datalog/lib &&
(cd examples/graph/graph_lib && cargo build --release && cargo run --release -- 200 2000) 

# Uncomment this line to update records from ddlog commandline language
# && (cd examples/graph/graph_ddlog && cargo build --release && target/release/graph_cli < ../graph.dat) 