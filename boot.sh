nasm -f bin -o boot.bin boot.asm
qemu-system-x86_64 -drive format=raw,file=boot.bin
