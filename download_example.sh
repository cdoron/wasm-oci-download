#!/bin/bash

if [ $# -ne 1 ];
    then echo "Usage: "$0" [yaml_filename]"
    exit
fi

cargo run $1
