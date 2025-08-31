#!/bin/bash

cd "$(cd "$(dirname "$0")" && pwd)"
echo "Now in script directory: $(pwd)"

chmod +x ./target/release/micro-monitor
./target/release/micro-monitor