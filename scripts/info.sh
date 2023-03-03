#!/bin/bash

# this script will use readelf to generate an `info.json` description of a 
# given elf file for all sections. Use it like this:
#   ./scripts/info.sh "assets/libvpf.so.4.1"

DATA=$(readelf -S $1);
DATA=${DATA//$'\n'/}

RESULT="[
    ";

# this ugly-ass regex is brought to you by the letter A. A as in "[_A-Z]+"
while [[ "$DATA" =~ \[[[:blank:]]*[[:digit:]]+\][[:blank:]]*([\.a-z]+)[[:blank:]]+([_A-Z]+)[[:blank:]]+0*([a-z0-9]*)[[:blank:]]+0*([a-z0-9]*)[[:blank:]]+([a-z0-9]+)[[:blank:]]+([a-z0-9]+) ]]; do
    FULL_MATCH=${BASH_REMATCH[0]}
    NAME_MATCH=${BASH_REMATCH[1]}
    TYPE_MATCH=${BASH_REMATCH[2]}
    ADDR_MATCH=${BASH_REMATCH[3]}
    OFFS_MATCH=${BASH_REMATCH[4]}
    SIZE_MATCH=${BASH_REMATCH[5]}
    ENTS_MATCH=${BASH_REMATCH[6]}

    NAME=$NAME_MATCH;
    TYPE=$TYPE_MATCH;
    ADDR=$(echo "ibase=16; ${ADDR_MATCH^^}" | bc);
    OFFS=$(echo "ibase=16; ${OFFS_MATCH^^}" | bc);
    SIZE=$(echo "ibase=16; ${SIZE_MATCH^^}" | bc);
    ENTS=$(echo "ibase=16; ${ENTS_MATCH^^}" | bc);
 
    RESULT="${RESULT}{
        \"name\": \"${NAME}\",
        \"type\": \"${TYPE}\",
        \"address\": \"${ADDR}\",
        \"offset\": \"${OFFS}\",
        \"size\": \"${SIZE}\",
        \"entsize\": \"${ENTS}\"
    },
    "

    DATA=${DATA/"${BASH_REMATCH[0]}"/}
done

RESULT="${RESULT%,}
]";
echo "$RESULT";