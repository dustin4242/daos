cargo bootimage --release
qemu-system-x86_64 -drive format=raw,file=target/x86_64-options/release/bootimage-daos.bin
