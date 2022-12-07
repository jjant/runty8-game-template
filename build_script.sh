#!/bin/bash

set -e

if [[ -z $1 ]]; then
  echo "Argument required, pass in your binary's name."
  echo "For example:"
  echo "  ./build_script runty8-game-template"
  exit 1
else
  package="$1"
fi

echo "Building: $package"

rm -rf generated/*
cargo build --target wasm32-unknown-unknown --bin "$package" --release
wasm-bindgen target/wasm32-unknown-unknown/release/$package.wasm --out-dir generated --target web

cp src/index.html generated/index.html
placeholder="__PACKAGE_NAME__"
echo "Replacing placeholder: $placeholder -> $package"
sed -i.bkp "s/$placeholder/$package/g" generated/index.html

cd generated
echo "Launching server:"
serve

