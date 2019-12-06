# RustOS
A for fun experiment with Rust.
Source of instructions: https://os.phil-opp.com/

This is still in development! Some things are bound to change with time. Check the actual tutorial linked above for the more recent up-to-date instructions.

## Dependencies
### Rust
The following are Rust dependencies that aren't automically installed.

To switch to nightly:

```rustup install nightly```

```rustup override set nightly```

Other components:

```rustup component add rust-src```

```rustup component add llvm-tools-preview```

Set rustup target: ```rustup target add thumbv7em-none-eabihf```
### Cargo
The following are Cargo dependencies that aren't automatically installed.


```cargo install cargo-xbuild```


```cargo install bootimage --version "^-.7.7"```

### QEMU
QEMU is light-weight virtual machine that is used to quickly debug and test the rust os. Follow this link to learn more: https://www.qemu.org/

## To Build
To build:

```cargo xbuild --target x86_64-rust_os.json```

Create a bootimage:

```cargo bootimage```

## Launching and Running
After building and installing all dependencies, one can quickly launch the os in a virtual machine by running:

```qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-RustOS.bin```
