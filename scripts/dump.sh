#!/bin/bash

# ./scripts/dump.sh "assets/libvpf.so.4.1" "000461c4" "107"

FILE=$1;
OFFSET=$(echo "ibase=16; ${2^^}" | bc);
LENGTH=$(echo "ibase=16; ${3^^}" | bc);

FORMAT='1/1 "%.2x "'

echo FILE=$FILE 
echo OFFSET=$OFFSET 
echo LENGTH=$LENGTH

RESULT=$(
    hexdump -ve '1/1 "%.2x "' \
        -s $OFFSET            \
        -n $LENGTH            \
        $FILE
);

ARRAY=($RESULT);

echo ${ARRAY[@]}