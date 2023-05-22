
# Introduction

> NOTE: This document is a working in progress, feel free to add or modify information in it to make it more up-to-date.

This source code is used to control the Circuit Board `Julia PCI`(version 1.10).

Part of the code is multiplatform agnostic and part is plataform specific (AVR microcontroler).

## Prerequisites

  * A recent version of the nightly Rust compiler. Anything including or
    greater than `rustc 1.63.0-nightly (fee3a459d 2022-06-05)` can be used.
    > for example this version is known to work: `rustc 1.67.0-nightly (53e4b9dd7 2022-12-04)`
  * The rust-src rustup component - `$ rustup component add rust-src`
  * AVR-GCC on the system for linking
  * AVR-Libc on the system for support libraries
  * gnu-make

## Usage

Before start building, enter terminal go to the directory project and type: 

```
> rustup toolchain install nightly-2022-12-04
```

then type:

```
> rustup default nightly-2022-12-04
> rustup override set nightly-2022-12-04
> rustup override nightly-2022-12-04
> rustup component add rust-src --toolchain nightly-2022-12-04-x86_64-pc-windows-msvc
```

If you want to install `cargo-watch` utility type this:

```
> cargo install cargo-watch
```

> Note: Some of upabove commands may be unnecessary, check and update this doc when possible.


This will install the specified version of the rust toolchain.

Finally tpe:

```
> make
``` 

The `make` command will run the Makefile rules that compile, upload code to the board and show the total size of your compiled program.
 
You can also type `make build upload size`, or if you prefer type only parts of this rules, exemple: `make upload` to just upload without compiling code, or `make build` to just compile code without upload it, etc.

To see all rules available see the `Makefile` in the project's main directory.


# Building manually (details)

> This section of this document must be improved

```bash
rustup override set nightly

# Ensure time delays are consistent with a 16MHz microcontroller.
export AVR_CPU_FREQUENCY_HZ=16000000

# Compile the crate to an ELF executable.
cargo build -Z build-std=core --target avr-atmega328p.json --release

# Note: there is work in progress to make that easier
#       peeking in .cargo/config.tom  will show that
#            cargo build --release
#       is the short version
```
There should now be an ELF file at `target/avr-atmega328p/release/blink.elf`. It
can be flashed directly to an AVR microcontroller or ran inside a simulator.


## Resources

  * The [AVR-Rust book](https://book.avr-rust.com)

