#!/bin/sh

set -ex

wasm-pack build -d public/pkg --target web 
#firefox localhost:8000 &
python3 -m http.server -d public
