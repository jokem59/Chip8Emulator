# README

This project serves as first attempt at building an emulator as well as my first project in Rust.

## Overview

I'll be utilizing this document to capture my lessons learned.

## Sources
[Chip8 Specifications](https://en.wikipedia.org/wiki/CHIP-8)
[How to build an emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)

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
