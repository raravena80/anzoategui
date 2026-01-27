# Anzoategui [![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/raravena80/anzoategui/master/LICENSE)

A Rust learning playground with practical code examples covering fundamental and advanced Rust concepts.

![anzoategui](https://user-images.githubusercontent.com/7659560/36757648-fee7479c-1bc6-11e8-9f11-0d909256a949.jpg)

## About

This repository is a collection of Rust examples I created while learning the language. Most concepts are derived from the official Rust documentation, but organized for easy exploration and hands-on learning.

**The name Anzoategui** is inspired by the beautiful state in Venezuela.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Running Examples](#running-examples)
- [Topics Covered](#topics-covered)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites

To run these examples, you need to have Rust installed on your system.

### Installing Rust

The recommended way to install Rust is using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For other installation methods, visit: https://www.rust-lang.org/tools/install

### Verify Installation

```bash
rustc --version
cargo --version
```

## Getting Started

Clone the repository:

```bash
git clone https://github.com/raravena80/anzoategui.git
cd anzoategui
```

Build all examples:

```bash
cargo build --all-targets
```

## Running Examples

This project is organized as a Cargo package with multiple binary examples. Each example can be run individually:

```bash
# Run a specific example
cargo run --bin <example-name>

# For example, to run the arrays example:
cargo run --bin arrays_arrays

# To run the borrowing mutable example:
cargo run --bin borrow_mut

# To run the traits example:
cargo run --bin traits_traits
```

### List All Available Examples

```bash
ls src/bin/
```

### Running Original Files

The original files are still available in their respective directories and can be compiled directly with `rustc`:

```bash
# Navigate to a topic directory
cd arrays

# Compile and run
rustc arrays.rs
./arrays
```

## Topics Covered

### Beginner Level

| Topic | Description | Example Files |
|-------|-------------|---------------|
| **Arrays** | Working with fixed-size collections | [`arrays/arrays.rs`](arrays/arrays.rs) |
| **Tuples** | Grouping values of different types | [`tuples/tuples.rs`](tuples/tuples.rs), [`tuples/tuples2.rs`](tuples/tuples2.rs) |
| **Loops** | Iteration patterns in Rust | [`loops/loops.rs`](loops/loops.rs) |
| **Range** | Using range expressions | [`range/range.rs`](range/range.rs) |
| **If Let** | Pattern matching shorthand | [`iflet/iflet.rs`](iflet/iflet.rs) |
| **While** | While loop patterns | [`while/while.rs`](while/while.rs) |
| **Enums** | Defining enumerated types | [`enum/enum.rs`](enum/enum.rs) |
| **Structs** | Creating custom data structures | [`structs/structs.rs`](structs/structs.rs) |
| **Display** | Implementing the Display trait | [`display/display.rs`](display/display.rs) |

### Intermediate Level

| Topic | Description | Example Files |
|-------|-------------|---------------|
| **Borrowing** | Understanding Rust's ownership system | [`borrow/mut.rs`](borrow/mut.rs), [`borrow/idiomatic.rs`](borrow/idiomatic.rs) |
| **Ownership** | Move semantics and ownership rules | [`ownership/clone.rs`](ownership/clone.rs), [`ownership/take.rs`](ownership/take.rs) |
| **Mutability** | Mutable vs immutable bindings | [`mutability/mut.rs`](mutability/mut.rs), [`mutability/int.rs`](mutability/int.rs) |
| **Slices** | Working with array slices | [`slices/slices.rs`](slices/slices.rs) |
| **Methods** | Defining methods on types | [`methods/methods.rs`](methods/methods.rs) |
| **Closures** | Anonymous functions and captures | [`closures/closure.rs`](closures/closure.rs), [`closures/capture.rs`](closures/capture.rs) |
| **Iterators** | Iterator patterns and adapters | [`iterator/iterator.rs`](iterator/iterator.rs) |
| **Pattern Guards** | Advanced pattern matching | [`guards/guards.rs`](guards/guards.rs) |
| **Type Coercion** | Implicit and explicit type conversion | [`coercion/coerce.rs`](coercion/coerce.rs) |
| **From/Into** | Conversion traits | [`fromto/from.rs`](fromto/from.rs), [`fromto/into.rs`](fromto/into.rs) |

### Advanced Level

| Topic | Description | Example Files |
|-------|-------------|---------------|
| **Lifetimes** | Managing reference lifetimes | [`lifetimes/longest.rs`](lifetimes/longest.rs) |
| **Aliasing** | Reference aliasing rules | [`aliasing/aliasing.rs`](aliasing/aliasing.rs), [`aliasing/forresult.rs`](aliasing/forresult.rs) |
| **Traits** | Defining shared behavior | [`traits/traits.rs`](traits/traits.rs), [`traits/traits2.rs`](traits/traits2.rs) |
| **Bounds** | Trait bounds and generic constraints | [`bounds/bounds.rs`](bounds/bounds.rs) |
| **Macros** | Metaprogramming with macros | [`macros/macro.rs`](macros/macro.rs) |
| **Threads** | Concurrent programming | [`threads/threads.rs`](threads/threads.rs) |
| **Channels** | Message passing between threads | [`channels/channels.rs`](channels/channels.rs) |
| **RAII** | Resource Acquisition Is Initialization | [`raii/raii.rs`](raii/raii.rs) |
| **Smart Pointers** | Box, Rc, and other pointer types | [`pointers/pointers.rs`](pointers/pointers.rs) |

### Language Features

| Topic | Description | Example Files |
|-------|-------------|---------------|
| **Module System** | Code organization with modules | [`use/use.rs`](use/use.rs), [`superself/superself.rs`](superself/superself.rs) |
| **File Hierarchy** | Multi-file projects | [`filehierarchy/split.rs`](filehierarchy/split.rs) |
| **Nesting** | Nested modules and scopes | [`nesting/nesting.rs`](nesting/nesting.rs) |
| **Cfg** | Conditional compilation | [`cfg/cfg.rs`](cfg/cfg.rs) |
| **Inference** | Type inference examples | [`inference/inference.rs`](inference/inference.rs) |
| **Declaration First** | Forward declarations | [`declarefirst/declarefirst.rs`](declarefirst/declarefirst.rs) |
| **Dead Code** | Handling unused code warnings | [`deadcode/deadcode.rs`](deadcode/deadcode.rs) |
| **Elision** | Lifetime elision rules | [`elision/elision.rs`](elision/elision.rs) |

### Data Structures

| Topic | Description | Example Files |
|-------|-------------|---------------|
| **HashMaps** | Key-value hash maps | [`hashmaps/hashmaps.rs`](hashmaps/hashmaps.rs) |
| **HashSet** | Unique value sets | [`hashset/hashset.rs`](hashset/hashset.rs) |
| **LinkedList** | Doubly-linked lists | [`linkedlist/linkedlist.rs`](linkedlist/linkedlist.rs) |

### I/O and System

| Topic | Description | Example Files |
|-------|-------------|---------------|
| **File I/O** | Reading and writing files | [`file/open.rs`](file/open.rs) |
| **Path** | Working with file paths | [`path/path.rs`](path/path.rs) |
| **Pipes** | Process pipes and streams | [`pipes/pipes.rs`](pipes/pipes.rs) |
| **Wait** | Process synchronization | [`wait/wait.rs`](wait/wait.rs) |

## Project Structure

```
anzoategui/
├── Cargo.toml              # Package manifest
├── src/
│   └── bin/                # Binary examples (runnable with cargo run --bin)
├── <topic>/                # Original organized directories
│   └── *.rs               # Source files
├── LICENSE                 # MIT License
└── README.md              # This file
```

## Building and Testing

### Build All Examples

```bash
cargo build --all-targets
```

### Check for Compilation Errors

```bash
cargo check
```

### Format Code

```bash
cargo fmt
```

### Run Clippy Lints

```bash
cargo clippy --all-targets
```

## Contributing

Contributions are welcome! If you'd like to add more examples or improve existing ones:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-example`)
3. Add your changes
4. Ensure code is formatted (`cargo fmt`)
5. Commit your changes (`git commit -am 'Add new example'`)
6. Push to the branch (`git push origin feature/new-example`)
7. Create a Pull Request

## Learning Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings - Small exercises](https://github.com/rust-lang/rustlings)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2017 Ricardo Aravena

## Acknowledgments

Most examples are based on concepts from the official Rust documentation and "Rust by Example". This repository is organized for personal learning and reference.

---

**Note**: Use these examples at your own risk. They are primarily educational and may not represent production-ready code patterns.
