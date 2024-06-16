#!/usr/bin/env bash
DIR=$(dirname $1)
mkdir -p $DIR/efi/boot
cp $1 $DIR/efi/boot/bootx64.efi

qemu-system-x86_64-uefi --enable-kvm -drive file=fat:rw:$DIR,format=raw -nodefaults -display gtk -vga std -cpu host -m 8192
