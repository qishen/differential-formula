#!/bin/bash

echo "Compiling `formula2ddlog` transformation into Rust runtime" 
# Build formula2ddlog Rust runtime from formula2ddlog.dl and compile it all the way to binary
ddlog -i ../formula2ddlog.dl -L ~/differential-datalog/lib/ && 
(cd ../formula2ddlog_lib && cargo build)
# (cd formula2ddlog_lib && cargo build --release)

echo "Generating DDLog program graph.dl from FORMULA program graph.4ml"
# Use `formula2ddlog` to transform graph.4ml into graph.dl
../formula2ddlog_lib/target/debug/formula2ddlog_lib graph/graph.4ml > graph/graph.dl &&

echo "Compiling DDLog program graph.dl to Rust runtime"
# If the std lib is missing we have to specify the path when generating Rust runtime for domain
ddlog -i graph/graph.dl -L ~/differential-datalog/lib/