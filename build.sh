#!/bin/bash
cargo build --release &&
rm -rf app/* ;
cp target/release/mild app/ &&
rm -rf target/ ;
echo "Mild was successfully compiled" ;