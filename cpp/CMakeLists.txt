cmake_minimum_required(VERSION 3.10)

# Project name
project(Chip8Emulator VERSION 1.0)

# Add exectuable
add_executable(Chip8Emulator main.cpp)

# Configure header file
configure_file(main.h.in main.h)
