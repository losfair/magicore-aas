#!/bin/bash

set -e

make -C ./primitives

cargo xbuild --release --target ./riscv64im-unknown-none-elf.json
cp ./target/riscv64im-unknown-none-elf/release/magicore-aas ./magicore-aas.elf
llvm-objdump -d -mattr=+m,+a ./magicore-aas.elf > magicore-aas.dump
