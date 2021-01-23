#!/bin/bash

PKG_DIR=target/package/ff6party
mkdir -p ${PKG_DIR}
cp -r static ${PKG_DIR}
cp target/release/ff6party.exe ${PKG_DIR}
rm target/package/ff6party.zip
(cd target/package; 7z a ff6party.zip ff6party)