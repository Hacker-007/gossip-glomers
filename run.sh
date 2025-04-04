#!/bin/bash

service=$1
workload=$2

clear && cargo build --bin $service --release
clear && ~/maelstrom/maelstrom test -w $workload --bin target/release/$service --node-count 25 --time-limit 20 --rate 100 --latency 100