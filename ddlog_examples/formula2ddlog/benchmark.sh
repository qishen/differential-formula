#!/bin/bash

echo '
Usage:
run.sh <NODE=2000> <EDGE=100> <TARGET=ddlog>
if the target is `ddlog` or `formula`, <NODE> and <EDGE> are the attributes of the auto-generated random graph.
if the target is for benchmark as `*_bench`, <NODE> is the same attribute for the auto-generated random graph 
but <EDGE> is number of both initial edges and the range interval.
'
NODE_NUM=${1-2000}
EDGE_NUM=${2-100}
TARGET=${3-ddlog}
echo "Print out bash script arguments: Nodes=$NODE_NUM, Edges=$EDGE_NUM, Target=$TARGET"

# Download DDLog-0.47.0 release rather than building from latest DDLog source code
if [ ! -d ddlog ]; then
wget https://github.com/vmware/differential-datalog/releases/download/v0.47.0/ddlog-v0.47.0-20210819223359-Linux.tar.gz -O ddlog.tar.gz \
	&& tar -xzvf ddlog.tar.gz && rm ddlog.tar.gz
fi

if [[ $TARGET = ddlog* ]]; then
	echo "Compiling `formula2ddlog` transformation into Rust runtime" 
	# Build formula2ddlog Rust runtime from formula2ddlog.dl and compile it all the way to binary
	ddlog/bin/ddlog -i formula2ddlog.dl -L ddlog/lib && 
	(cd formula2ddlog_lib && cargo build --release)

	echo "Generating DDLog program `graph.dl` from FORMULA program `graph.4ml`"
	# Use `formula2ddlog` to transform graph.4ml into graph.dl
	formula2ddlog_lib/target/release/formula2ddlog_lib examples/graph/graph.4ml > examples/graph/graph.dl &&

	echo "Compiling DDLog program `graph.dl` to Rust runtime"
	# If the std lib is missing we have to specify the path when generating Rust runtime for domain
	ddlog/bin/ddlog -i examples/graph/graph.dl -L ddlog/lib

	pushd examples/graph/graph_lib > /dev/null
	if [[ $TARGET = "ddlog_bench" ]]; then
		echo "Start DDLog benchmark starting from $NODE_NUM nodes and $EDGE_NUM edges"
		cargo run --release -- $NODE_NUM $EDGE_NUM ddlog_bench
	elif [[ $TARGET = "ddlog_manual" ]]; then
		echo "Running DDLog graph computation with inputs from `graph.dat` incrementally"
		pushd ../graph_ddlog > /dev/null
		# Update records from ddlog commandline language
		cargo build --release && target/release/graph_cli < ../graph.dat
		popd
	else 
		echo "Running DDLog graph computation once on a random graph with $NODE_NUM nodes and $EDGE_NUM edges"
		cargo run --release -- $NODE_NUM $EDGE_NUM ddlog
	fi
	popd > /dev/null

elif [[ $TARGET = formula* ]]; then

	# Copy noninteractive executable from archive to the folder that contains all FORMULA files for testing
	cp -r ../../archive/executable_noninteractive/* examples/graph/files/

	pushd examples/graph/files/ > /dev/null
	if [[ $TARGET = "formula_bench" ]]; then
		echo "Clean folder and generate .4ml files for benchmark starting from $NODE_NUM nodes and $EDGE_NUM edges"
		rm *.4ml
		pushd ../graph_lib > /dev/null
		cargo run --release -- $NODE_NUM $EDGE_NUM formula_bench
		popd
		for FILE in $(find . -maxdepth 1 -type f -name \*.4ml | sort); do 
			load_start=`date +%s%3N`
			dotnet CommandLine.dll "load $FILE" > /dev/null
			load_end=`date +%s%3N`
			load_time=`expr $load_end - $load_start`

			start=`date +%s%3N`
			dotnet CommandLine.dll "load $FILE | qr m Path(a, b)" > /dev/null
			end=`date +%s%3N`
			execution_time=`expr $end - $start`
			# The real execution time needs to minus the loading time
			echo "Execution time of $FILE is `expr $execution_time - $load_time` milliseconds"
		done
	else 
		echo "Clean folder and generate one .4ml file for testing"
		rm *.4ml
		pushd ../graph_lib > /dev/null
		cargo run --release -- $NODE_NUM $EDGE_NUM formula
		popd
		dotnet CommandLine.dll "load graph_n${NODE_NUM}_e${EDGE_NUM}.4ml | qr m Path(a, b)"
	fi
	popd > /dev/null
fi

