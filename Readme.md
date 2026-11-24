# Brainfuck Interpreter

A Brainfuck interpreter implemented in **Rust**, built to explore virtual machine architecture, instruction execution, memory management, and control flow.

## Overview

This project implements a simple virtual machine capable of executing Brainfuck programs. It uses a memory tape, data and instruction pointers, and a precomputed jump table to handle loop instructions efficiently.

## Features

- **30,000-cell memory tape** using `u8` cells.
- **8 Brainfuck instructions** supported.
- **Wrapping arithmetic** for cell increment and decrement.
- **Memory pointer boundary checking**.
- **Precomputed jump table** for efficient loop execution.
- **Nested loop support**.
- **Bracket validation** to detect unmatched opening and closing brackets.
- **Error propagation** using Rust's `Result` and `?` operator.

## Supported Instructions

| Instruction | Operation |
|:---:|---|
| `>` | Move data pointer right |
| `<` | Move data pointer left |
| `+` | Increment current cell |
| `-` | Decrement current cell |
| `.` | Output current cell as a character |
| `,` | Read one byte of input |
| `[` | Jump forward if current cell is zero |
| `]` | Jump backward if current cell is nonzero |

All characters other than these eight instructions are ignored.

## Project Structure

```text
brainfuck/
├── src/
│   ├── main.rs
│   └── vm.rs
├── Cargo.toml
└── README.md
```

- `main.rs` — Program entry point and VM initialization.
- `vm.rs` — Virtual machine implementation, instruction execution, memory management, and jump-table construction.

## Getting Started

### Prerequisites

- Rust
- Cargo

### Clone the repository

```bash
git clone <repository-url>
cd brainfuck
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

The Brainfuck source program is currently supplied through the VM initialization in the Rust source code.

## Example

Brainfuck program:

```brainfuck
++++++++++[>+++++++>++++++++++>+++>+<<<<-]
>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

This is a classic Brainfuck program that prints:

```text
Hello World!
```

## Implementation Details

### Memory Model

The VM uses a fixed-size memory tape containing 30,000 cells. Each cell is an unsigned 8-bit integer (`u8`).

Arithmetic operations use Rust's `wrapping_add()` and `wrapping_sub()` methods, allowing cell values to wrap around between 0 and 255.

### Instruction Execution

The interpreter maintains two pointers:

- **Data Pointer (`DP`)** — Tracks the current memory cell.
- **Instruction Pointer (`IP`)** — Tracks the current instruction being executed.

Instructions are fetched sequentially and dispatched to their corresponding operations.

### Loop Handling

Instead of searching for matching brackets during execution, the VM constructs a jump table before running the program.

A stack is used to match opening and closing brackets, supporting nested loops and detecting malformed programs before execution begins.

## Error Handling

The VM currently detects:

- Unmatched opening brackets.
- Unmatched closing brackets.
- Memory pointer boundary violations.

Execution errors are propagated using Rust's `Result` type.

## License

See [LICENSE](LICENSE) for licensing information.