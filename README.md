# daos
Dustin's Awesome Operating System! (Kinda Sucks Rn But I'll Work On It At Some Point)

Requires: `qemu-system-x86` & `rust`

## Note:
You may need a ui frontend for qemu, for instance on my system the aur has a package called `qemu-ui-gtk` for a graphical interface.

Because of rust's stupidness we need to do a few things to setup after cloning the repo:
> set toolchain to nightly via `rustup override set nightly`

> install 2 things via cargo: `bootimage` & `llvm-tools-preview`

then you should be set.