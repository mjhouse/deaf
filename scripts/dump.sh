#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "USAGE: ./scripts/dump.sh \"path/to/lib.so\" \".section.name\"";
    exit 1;
fi


DATA=$(readelf -S $1);
DATA=${DATA//$'\n'/};

# get the basename of the given file
BASENAME=$(basename $1);
BASENAME=${BASENAME}

# get up to the first period in the name
[[ $BASENAME =~ ^([a-zA-Z0-9]+) ]]
BASENAME=${BASH_REMATCH[1]}

# create the output location for the dump
mkdir -p assets/${BASENAME}/dump/;

# copy the library if it doesn't exist
cp -n $1 assets/${BASENAME};

INFO="[\n\t";

# this ugly-ass regex is brought to you by the letter A. A as in "[_A-Z]+"
while [[ "$DATA" =~ \[[[:blank:]]*[[:digit:]]+\][[:blank:]]*([\._a-zA-Z]+)[[:blank:]]+([_A-Z]+)[[:blank:]]+0*([a-z0-9]*)[[:blank:]]+0*([a-z0-9]*)[[:blank:]]+([a-z0-9]+)[[:blank:]]+([a-z0-9]+) ]]; do
    
    # parse matches from regular expression
    FULL_MATCH=${BASH_REMATCH[0]}
    NAME_MATCH=${BASH_REMATCH[1]}
    TYPE_MATCH=${BASH_REMATCH[2]}
    ADDR_MATCH=${BASH_REMATCH[3]}
    OFFS_MATCH=${BASH_REMATCH[4]}
    SIZE_MATCH=${BASH_REMATCH[5]}
    ENTS_MATCH=${BASH_REMATCH[6]}

    # advance to the next regex match in the data
    DATA=${DATA/"${BASH_REMATCH[0]}"/}

    # convert to desired base / format
    NAME="section${NAME_MATCH//./_}";
    TYPE=$TYPE_MATCH;
    ADDR=$(echo "ibase=16; ${ADDR_MATCH^^}" | bc);
    OFFS=$(echo "ibase=16; ${OFFS_MATCH^^}" | bc);
    SIZE=$(echo "ibase=16; ${SIZE_MATCH^^}" | bc);
    ENTS=$(echo "ibase=16; ${ENTS_MATCH^^}" | bc);

    # add section info to the json output
    INFO="${INFO}{
        \"name\": \"${NAME_MATCH}\",
        \"type\": \"${TYPE}\",
        \"address\": \"${ADDR}\",
        \"offset\": \"${OFFS}\",
        \"size\": \"${SIZE}\",
        \"entsize\": \"${ENTS}\"
    },
    "
    # only write out bytes for the requested section
    if [ "$NAME_MATCH" = "$2" ]; then

        # build a path to the dump file for the section
        FILENAME=assets/${BASENAME}/dump/${NAME}.in

        # extract hex for the target secion
        RESULT=$(hexdump -ve '1/1 "%.2x "' -s $OFFS -n $SIZE $1);
        ARRAY=($RESULT);

        # default to a set width if no entity size found
        if [ "$ENTS" = "0" ]; then
            ENTS=16
        fi

        # build a valid rust array to output
        OUTPUT="&[";
        for i in ${!ARRAY[@]}; do 
            if ! (( $i % $ENTS )); then 
                OUTPUT="${OUTPUT}\n\t";
            fi
            OUTPUT="${OUTPUT}0x${ARRAY[$i]}, ";
        done
        OUTPUT="${OUTPUT}\n]";

        # dump the rust array to a file
        echo -e "$OUTPUT" > ${FILENAME};
    fi
done

INFO="${INFO%,}\n]";

# build a path to the info file
FILENAME=assets/${BASENAME}/info.json
echo -e "$INFO" > ${FILENAME};