#!/bin/bash
set -e
RUST_BACKTRACE=1

KillJobs() {
    for job in $(jobs -p); do
            kill -s SIGTERM $job > /dev/null 2>&1 || (sleep 10 && kill -9 $job > /dev/null 2>&1 &)

    done
}

TrapQuit() {
    # Whatever you need to clean here
    KillJobs
}

trap TrapQuit EXIT

cargo build
cargo run &
cargo test
