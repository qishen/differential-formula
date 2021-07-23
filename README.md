# differential-formula
Incremental Formal Modeling Using Logic Programming and Analysis

## Installation
1. Install [Rust](https://www.rust-lang.org/tools/install) toolchain and [Differential-Datalog](https://github.com/vmware/differential-datalog) following the tutorials in the links. Use command `ddlog --version` to check if DDLog is successfully installed in the local environment. Maintain a copy of **Differential-Datalog** source code so we can call its standard library later.

2. Go to `ddlog_examples/formula2ddlog` and run `ddlog -i formula2ddlog.dl`. If DDLog cannot find the dependencies try `ddlog -i formula2ddlog.dl -L /path/to/differential-datalog/lib/` with additional argument to specify the path of standard library. After that DDLog generates a specific Rust runtime in `ddlog_examples/formula2ddlog/formula2ddlog_ddlog` for `formula2ddlog.dl` that provides DDLog Rust APIs for incremental computation based on **differential-datalog** 

## Testing
1. Go to `ddlog_examples/formula2ddlog/formula2ddlog_lib` and run `cargo run` to transform the model of a FORMULA program `formula2ddlog/examples/graph/graph.4ml` into the model of an equivalent DDLog program. The model of DDLog program is then translated into a real DDLog program and printed out in the command line.

2. Copy the DDLog program into `ddlog_examples/formula2ddlog/example/graph.dl` and run `ddlog -i graph.dl` to generate a Rust runtime for the graph domain that can execute constraint checking and model transformation incrementally.