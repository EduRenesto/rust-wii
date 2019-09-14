# rust-wii: running Rust code on the Wii

DISCLAIMER: This is mostly an experimental attempt. Do not expect proper
support, since I actually don't really know what I'm doing. I'm just trying
stuff until something works!

## Requirements

You'll need:
- A nightly Rust toolchain
- [Xargo](https://github.com/japaric/xargo)
- DevkitPRO (powerpc-eabi-{gcc, ar} must be in your PATH)
- A bit of patience

## Quick Start

1. Create a Cargo static library project normally
2. Copy the `powerpc-eabi.json` file to the root of your crate
3. Add the bindings to the Wii APIs as your dependences
4. Copy the bootstrap folder to the root of your crate
5. In the bootstrap folder, modify the Makefile to mention the name of your
   library
6. Call your main function `rust_main`, and make it return a i32 (just like the
   C main we're used to)
7. Run `make` in the bootstrap folder
8. Enjoy your .elf/.dol :D

## How it works

Before we start, please remind that this project is a work in progress, and it
is in a very early stage. So, things might change drastically!!

We instruct Cargo to build a static library with the PowerPC machine as a
target. The library exports a function called `rust_main`, which will be called
from the bootstraper C code (which resides in the bootstrap folder). 

We, then, use the DevkitPPC gcc to link our Rust static library (which is, at
this stage, a .a file) together with the bootstrap code and the needed
libraries. 

The rest of the process goes as usual, baking the .elf file into a .dol file.
From here, you can load it through HBC, any exploit, WiiLoad, and so on. It
also works on the Dolphin Emulator.

## Thanks

Thanks to

- The DevkitPRO team
- WiiBrew
- my friends GrayJack and Soarxyn for helping and enduring my hype-driven flood
  of messages
