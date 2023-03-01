#!/bin/bash

# list sections:
#       readelf -S assets/libvpf.so.4.1
# dump bytes
#       ./scripts/dump.sh "assets/libvpf.so.4.1" "000461c4" "107" 24 > assets/bytes/libvpf_strtab.in
#       ./scripts/dump.sh "assets/libvpf.so.4.1" "ab8" "1b90" 24 > assets/bytes/libvpf_symtab.in

#       ./scripts/dump.sh "assets/libvpf.so.4.1" "3940" "4c8" 24 > assets/bytes/libvpf_rela.dyn.in
#       ./scripts/dump.sh "assets/libvpf.so.4.1" "3e08" "540" 24 > assets/bytes/libvpf_rela.plt.in

#       ./scripts/dump.sh "assets/libvpf.so.4.1" "45b30" "8" 8 > assets/bytes/libvpf_init_array.in
#       ./scripts/dump.sh "assets/libqscintilla2_qt5.so.15.0.0" "3a16e8" "390" 8 > assets/bytes/libqscintilla_init_array.in

FILE=$1;
OFFSET=$(echo "ibase=16; ${2^^}" | bc);
LENGTH=$(echo "ibase=16; ${3^^}" | bc);
WIDTH=$4;

FORMAT='1/1 "%.2x "'

RESULT=$(
    hexdump -ve '1/1 "%.2x "' \
        -s $OFFSET            \
        -n $LENGTH            \
        $FILE
);

ARRAY=($RESULT);

printf "&[";
for i in ${!ARRAY[@]}; do 
    if ! (( $i % $WIDTH )); then 
        printf "\n\t";
    fi
    printf "0x${ARRAY[$i]}, ";
done
printf "\n]"