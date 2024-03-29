#!/bin/bash
IMAGE_NAME="turo"

# This is the script that `cargo test` will run. Before this script is invoked, `cargo test` will compile the
# kernel with testing code. After doing so, `cargo` will call this script with the path to the compiled test
# binary as an argument. 

# Before beginning, rename the `turo` file to kernel.elf
cp -v "$1" kernel.elf

# The first thing that we need to do is link the kernel with the limine bootloader and produce a . # iso file
rm -rf iso_root
mkdir -p iso_root
cp -v kernel.elf \
    ../limine.cfg ../limine/limine-bios.sys ../limine/limine-bios-cd.bin ../limine/limine-uefi-cd.bin iso_root/
mkdir -p iso_root/EFI/BOOT
cp -v ../limine/BOOTX64.EFI iso_root/EFI/BOOT/
cp -v ../limine/BOOTIA32.EFI iso_root/EFI/BOOT/
xorriso -as mkisofs -b limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_root -o "$IMAGE_NAME".iso
../limine/limine bios-install "$IMAGE_NAME".iso
rm -rf iso_root

# After we link the kernel with the `limine` bootloader, we need to run the tests on qemu
# Adding timeout command with a specified duration (e.g., 30 seconds)
timeout --foreground 30 qemu-system-x86_64 -serial stdio -M q35 -m 8G -smp 4 -cdrom "$IMAGE_NAME".iso -boot d -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none

# Then, we must get the exit code of `qemu` and map it to a 0 | 1 for `cargo test`
exit_code=$?

# Check if the exit code is 33 (hardcoded in the kernel)
if [ $exit_code -eq 33 ]; then
    exit 0
# Check if the timeout was triggered (exit status 124 from `timeout` command)
elif [ $exit_code -eq 124 ]; then
    echo "The test timed out."
    exit 1
else
    exit 1
fi