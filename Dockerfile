FROM rust:1.54 AS builder
RUN apt-get update && apt-get install -y wget \
	&& rm -rf /var/lib/apt/lists/*
# Copy differential-formula into docker working directory
WORKDIR /differential-formula
COPY . .

# Change working directory to `ddlog_examples/formula2ddlog` and download ddlog-0.47.0 binary
WORKDIR /differential-formula/ddlog_examples/formula2ddlog

RUN wget https://github.com/vmware/differential-datalog/releases/download/v0.47.0/ddlog-v0.47.0-20210819223359-Linux.tar.gz -O ddlog.tar.gz \
	&& tar -xzvf ddlog.tar.gz && rm ddlog.tar.gz

# Use ddlog binary to generate Rust runtime library for `formula2ddlog.dl` and compile the transformation
# tool `formula2ddlog_lib` all the way to the binary. The executable named `formula2ddlog_lib` will be 
# copied to the final stage later
RUN ddlog/bin/ddlog -i formula2ddlog.dl -L ddlog/lib && \ 
	(cd formula2ddlog_lib && cargo build --release)

# Use the `formula2ddlog_lib` executable to generate `graph.dl` from `graph.4ml` for graph domain
RUN formula2ddlog_lib/target/release/formula2ddlog_lib examples/graph/graph.4ml > examples/graph/graph.dl 

# Use ddlog binary to generate Rust runtime library for `graph.dl` and compile the benchmarking tool
# `graph_lib` all the way to the binary. The executable named `graph_lib` will be copied to the final
# stage later
RUN ddlog/bin/ddlog -i examples/graph/graph.dl -L ddlog/lib && \
	(cd examples/graph/graph_lib && cargo build --release)


FROM ubuntu:20.04
WORKDIR /dformula-executable
RUN apt-get update && apt-get install -y \
	apt-transport-https \
	wget

# Install .NET dependencies to run legacy FORMULA in Linux environment
RUN wget https://packages.microsoft.com/config/ubuntu/20.04/packages-microsoft-prod.deb -O packages-microsoft-prod.deb
RUN dpkg -i packages-microsoft-prod.deb && rm packages-microsoft-prod.deb
RUN apt-get update && apt-get install -y \
	dotnet-sdk-5.0 \ 
	aspnetcore-runtime-5.0 \
	&& rm -rf /var/lib/apt/lists/*

# Copy all legacy formula executable from building stage
COPY --from=builder /differential-formula/archive/ archive
# Copy ddlog-0.47.0 binary to working directory
COPY --from=builder /differential-formula/ddlog_examples/formula2ddlog/ddlog/ ddlog
# Copy `formula2ddlog_lib` executable that takes a FORMULA file as parameter and output the ddlog program 
COPY --from=builder /differential-formula/ddlog_examples/formula2ddlog/formula2ddlog_lib/target/release/formula2ddlog_lib  formula2ddlog_lib
# Copy `graph_lib` executable that takes some parameters to generate
COPY --from=builder /differential-formula/ddlog_examples/formula2ddlog/examples/graph/graph_lib/target/release/graph_lib  graph_lib
