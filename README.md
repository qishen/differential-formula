# differential-formula
Incremental Formal Modeling Using Logic Programming and Analysis

## Install Dependencies
1. Install [Rust](https://www.rust-lang.org/tools/install) toolchain, [.NET SDK and Runtime](https://docs.microsoft.com/en-us/dotnet/core/install/linux-ubuntu) and [Differential-Datalog](https://github.com/vmware/differential-datalog) following the tutorials in the links. Use command `ddlog --version` to check if DDLog is successfully installed in the local environment. Maintain a copy of **Differential-Datalog** source code so we can call its standard library later.

## Building
1. Generate a Rust Runtime for `formula2ddlog` transformation tool 
	- Go to directory `ddlog_examples/formula2ddlog` and run `ddlog -i formula2ddlog.dl`. If DDLog cannot find the dependencies try `ddlog -i formula2ddlog.dl -L /path/to/differential-datalog/lib/` with additional argument to specify the path of standard library. 
	- The Rust runtime for `formula2ddlog` is generated in directory `ddlog_examples/formula2ddlog/formula2ddlog_ddlog` based on the DDLog program `formula2ddlog.dl` that formally specifies the transformation rules from FORMULA to DDLog.
	- Build the Rust runtime directly if you want to use the raw transformation tool in command line by running `cargo build --release` inside `formula2ddlog_ddlog`. The command line only accepts data written in DDLog format.

2. Build the `formula2ddlog_lib` with command `cargo run --release` which will use the Rust runtime built from `formula2ddlog.dl` as library and generate an executable for transformation tool. This tool can translate any FORMULA file such as `formula2ddlog/examples/graph/graph.4ml` in our example into an equivalent DDLog program in `graph.dl` in the same folder. 


3. Copy the DDLog program into `ddlog_examples/formula2ddlog/example/graph/graph.dl` and run `ddlog -i graph.dl` to generate a Rust runtime for the graph domain that can execute constraint checking and model transformation incrementally. The Rust runtime for graph domain is generated in `graph_ddlog` and a separate `graph_lib` uses the Rust runtime in `graph_ddlog` as library to build an executable that does graph domain related computation incrementally.

## Testing
Run `ddlog_examples/formula2ddlog/run.sh` to benchmark constraint checking on random graph in both FORMULA and DDLog.

```
Usage:
run.sh <NODE=2000> <EDGE=100> <TARGET=ddlog>
```

if the target is set to `ddlog` or `formula`, <NODE> and <EDGE> are the attributes of the auto-generated random graph. The constraint checking will only be executed once on the random graph with the exact size we specify in the arguments or the default value.

if the target is for benchmark as `ddlog_bench` or `formula_bench`, <NODE> is the number of nodes for the auto-generated random graph but <EDGE> is number for both initial edges and the range interval. The constraint checking will be executed multiple times for the benchmarking.