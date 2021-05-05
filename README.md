# GDB FunEnd
A fun gdb frontend!
# Why?
First time I really needed GDB was when I was debugging the generated code from [a compiler that I was writing](https://github.com/gbrls/rust-microc).
It work wonders to aid me in the debugging process, but the interface was really bad.
I wanted to inspect memory, the registers and step instruction by instruction, gdb makes all of this possible, but not in a nice way.
# How?
Recently I came across the idea of [Tools for Thought](https://numinous.productions/ttft/) which can be distilled as:
> A tool that helps your thinking process, e.g: The Hindu–Arabic numeral system and Adobe Photoshop.

So, this project is an attempt to create a tool for thought that helps people to reason more deeply about the workings of computer programs.
# What?
Right now we have a vertical slice of the project (we have a thread running GDB that communicates to a thread that displays the source code).
### Why Rust?
I first considered writing this in C++, but then I thought about writing multithreaded code in it.

# Contributing
If you want to help you need to setup the developing enviroment.
- Install the rust toolchain https://rustup.rs/
- Clone the repository
- Build this project's documentation `cargo doc --no-deps --open`
