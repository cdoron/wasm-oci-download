#!/bin/bash

if [ $# -ne 3 ];
    then echo "Usage: "$0" [username] [password] [wasm-oci-image]"
    exit
fi

cargo run --example download-image -- $1 $2 $3
