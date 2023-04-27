#!/bin/bash
cd source/virtualization/ &&
cargo build --release &&
cd .. &&
cd .. &&
rm -rf bin/* ;
cp source/virtualization/target/release/virtualization bin/ &&
rm -rf source/virtualization/target/ ;
mv bin/virtualization bin/virtualization.smild
echo "Virtualization submodule package was successfully compiled" ;