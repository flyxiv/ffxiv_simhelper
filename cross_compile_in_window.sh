#!/bin/bash

brew install mingw-w64

rustup target add x86_64-pc-windows-gnu
cargo build --target=x86_64-pc-windows-gnu --release
