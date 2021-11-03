#!/bin/bash

if [ $# -ne 1 ];
    then echo "Usage: "$0" [yaml_filename]"
    exit
fi

export CREDENTIALS="{\"auths\":{\"ghcr.io\":{\"username\":\"the-mesh-for-data\",\"password\":\"XXX\",\"auth\":\"YYY\"}}}"

cargo run $1
