#!/bin/bash
RUST_BACKTRACE=1 cargo run && ./debug_link.sh && ./tmp/prog
