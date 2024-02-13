# daos
Dustin's Awesome Operating System! (Kinda Sucks Rn But I'll Work On It At Some Point)

Requires: `qemu-system-x86` & `rust`

## To Run:

1. Because of rust's stupidness we need to do a few things to setup after cloning the repo:
> Set toolchain to nightly via `rustup override set nightly`

> Install 2 things via cargo: `bootimage` & `llvm-tools-preview`
> *You can install llvm-tools via `rustup component add llvm-tools-preview`*

2. By default I have the `daos-lib` set to the github repository in the `Cargo.toml`, however if this changes to `"../daos-lib"` all that means is that you need to clone the repository to the same folder that `daos` is in.

3. After all that, to run we use the `./boot.sh` file.

## Note:
You may need a ui frontend for qemu, for instance on my system the aur has a package called `qemu-ui-gtk` for a graphical interface.
