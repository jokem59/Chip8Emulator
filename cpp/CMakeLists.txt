cmake_minimum_required(VERSION 3.10)

# Project name
project(Chip8Emulator VERSION 1.0)

# Add exectuable
add_executable(Chip8Emulator main.cpp)

# Specify C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQRUIED True)
