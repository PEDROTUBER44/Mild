#!/bin/sh

cargo build --release ;
rm -rf app/* ;
cp target/release/mild app/ ;
rm -rf target/ ;
echo "Mild successfully compiled" ;