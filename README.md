# README

This project serves as first attempt at building an emulator as well as my first project in Rust.

## Overview

I'll be utilizing this document to capture my lessons learned.

## Resources
[Wikipedia Chip8 Specifications](https://en.wikipedia.org/wiki/CHIP-8)
[Devernay Chip8 Sepcifications](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#font)
[How to build an emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
[Misc helpful Chip8 tips](https://github.com/AfBu/haxe-CHIP-8-emulator/wiki/(Super)CHIP-8-Secrets#understanding-of-store-bcd-instruction)

## Design Planning

The initial project will have everything in one file for simplicity.  After a working example is created, the project will be split into appropriate modules.

**Memory**
4096 bytes (0x1000)

512 bytes  | 0x0000 - 0x01FF: Used for the interpreter
3232 bytes | 0x0200 - 0x0E9F: Program memory
96 bytes   | 0x0EA0 - 0x0EFF: Reserved for call stack, internal use, other variables
256 bytes  | 0x0F00 - 0x0FFF: Reserved for display refresh

**Registers**


## Rust Lessons Learned


## Emulator Build Lessons Learned

**How is the Font Set defined?**

Q: I see in other [rust Chip8 implementations](https://github.com/starrhorne/chip8-rust/blob/master/src/font.rs) that there are fontsets defined, but I have no idea where theyre coming from.

A: See [Devernay Chip8 Sepcifications](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#font), Seciton 2.4 Display.  The fonts are sprites that represent the hexadecimal digits 0 - F.  Each sprite is 5 bytes long, or 8x5 pixels.  This data is stored in Chip-8 memory (0x0000 - 0x01FF) which is reserved for the interpreter.  The fontset that's provided defines 0 - F.  Custom fontsets can be created using the same byte patterns.

