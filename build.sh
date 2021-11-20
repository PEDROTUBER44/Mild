#!/bin/sh

cargo build --release ;
rm -rf app/* ;
cp target/release/mild app/ ;
rm -rf target/ ;
cp app/mild dev/debbuild/mild_1.0_amd64/usr/bin/ ;
echo "<<<>>>" ;