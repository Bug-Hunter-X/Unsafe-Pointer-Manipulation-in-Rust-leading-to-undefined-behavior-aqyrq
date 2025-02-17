# Unsafe Pointer Manipulation in Rust

This repository demonstrates a common pitfall when working with raw pointers in Rust. Directly manipulating memory via pointers can lead to undefined behavior if not handled extremely carefully. This example shows how modifying a vector's contents through a raw pointer after the vector goes out of scope can cause unexpected results or crashes.  The solution illustrates safer alternatives using Rust's ownership and borrowing system.

**Note:** Always prefer safe Rust over unsafe Rust whenever possible.  Unsafe code should only be used when absolutely necessary and with a deep understanding of its implications.