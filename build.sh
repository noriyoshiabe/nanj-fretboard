#!/bin/bash

set -xe

wasm-pack build --target web --release

rm -rf dist/
mkdir -p dist/pkg/

cp pkg/nanj_fretboard.js dist/pkg/
cp pkg/nanj_fretboard_bg.wasm dist/pkg/

cp public/apple-touch-icon.png dist/
cp public/favicon.ico dist/

cp index.html dist/

HASH_JS=`sha1sum dist/pkg/nanj_fretboard.js | awk '{print $1}'`
HASH_WASM=`sha1sum dist/pkg/nanj_fretboard_bg.wasm | awk '{print $1}'`

sed -i "" "s/nanj_fretboard.js/nanj_fretboard.js?h=${HASH_JS}/g" dist/index.html
sed -i "" "s/nanj_fretboard_bg.wasm/nanj_fretboard_bg.wasm?h=${HASH_WASM}/g" dist/pkg/nanj_fretboard.js
