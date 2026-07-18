#!/bin/bash

function run() {
    echo "$" "$@"
    "$@" || exit $?
}

min_version=1.81
max_version=1.97.1

for toolchain in $min_version $max_version; do
    run cargo +$toolchain --locked test
    run cargo +$toolchain --locked test --features=image
    for example in compressor decompressor downscale image rgb2rgb; do
        run cargo +$toolchain run --example $example --features=image
    done
done

run cargo +$max_version clippy --all --all-targets
