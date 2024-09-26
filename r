#!/usr/bin/env bash
rustc -o exe "$1" &&
./exe
