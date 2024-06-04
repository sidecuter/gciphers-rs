#!/usr/bin/bash

CARGO_HOME=$1
RUN="$2 $3 $4 $5 $6 $7 && cp $8 $9"
echo $RUN
$2 $3 $4 $5 $6 $7 $8 && cp $9 $10
