#!/bin/bash
cd $(dirname $1)
qemu-system-x86_64 --enable-kvm -bios /usr/share/ovmf/x64/OVMF.fd -drive file=fat:rw:.,format=raw -nodefaults -display gtk -vga std -cpu host -m 8192
