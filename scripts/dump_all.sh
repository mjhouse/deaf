#!/bin/bash

# This script just runs dump.sh to generate a bunch of output. It's a shortcut
# for iterating on the actual dump.sh script.

./scripts/dump.sh "assets/libjpeg/libjpeg.so.9" ".dynsym"

./scripts/dump.sh "assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0" ".init_array"
./scripts/dump.sh "assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0" ".fini_array"

./scripts/dump.sh "assets/libvpf/libvpf.so.4.1" ".dynsym"
./scripts/dump.sh "assets/libvpf/libvpf.so.4.1" ".init_array"
./scripts/dump.sh "assets/libvpf/libvpf.so.4.1" ".rela.dyn"
./scripts/dump.sh "assets/libvpf/libvpf.so.4.1" ".rela.plt"
./scripts/dump.sh "assets/libvpf/libvpf.so.4.1" ".shstrtab"