#!/bin/sh

mkdir -p tmp
mv ./prog_out.s ./tmp/prog_out.s
gcc src/be/platform/fig_runtime.c ./tmp/prog_out.s -o ./tmp/prog
