# Rust Virtual Machine

This project implements a register-based virtual machine in Rust that can execute custom assembly programs.

## Features
- Register-based architecture with 4 general-purpose registers (R0-R3)
- Custom assembly language (.orus files)
- Command-line interface for loading and executing programs
- Step-by-step execution tracing

## Getting Started

### Prerequisites
- Rust and Cargo installed

### Installation
```bash
git clone https://github.com/yourusername/rust_vm.git
cd rust_vm
cargo build
```

### Running Programs
```bash
cargo run path/to/program.orus
```

### Example Program
The repository includes a sample program that calculates the factorial of 5:

```assembly
// sample.orus - Factorial calculation
LOAD_CONST R0, 5  // n = 5
LOAD_CONST R1, 1  // result = 1
LOAD_CONST R2, 1  // constant 1

loop:
MUL R1, R0        // result = result * n
SUB R0, R2        // n = n - 1
JMP_IF_NOT_ZERO R0 loop  // loop while n != 0

PRINT_REG R1      // print result
HALT
```

To run this program:
```bash
cargo run sample.orus
```

## Assembly Language Reference

### Instructions
| Instruction | Parameters | Description |
|-------------|-----------|-------------|
| LOAD_CONST | reg, value | Load constant value into register |
| ADD | reg1, reg2 | Add reg2 to reg1 (reg1 = reg1 + reg2) |
| SUB | reg1, reg2 | Subtract reg2 from reg1 (reg1 = reg1 - reg2) |
| MUL | reg1, reg2 | Multiply reg1 by reg2 (reg1 = reg1 * reg2) |
| JMP_IF_NOT_ZERO | reg, label | Jump to label if register is not zero |
| PRINT_REG | reg | Print register value to console |
| HALT | | Stop program execution |

### Syntax Notes
- Comments start with `//`
- Labels end with `:` and can be used as jump targets
- Register names are case-insensitive (R0, r0, etc.)
- Values can be positive or negative integers

## Project Structure
- `src/main.rs` - Entry point and command-line handling
- `src/vm/mod.rs` - Virtual machine module
- `src/vm/machine.rs` - VM implementation
- `src/vm/instruction.rs` - Instruction set definition
- `src/vm/assembler.rs` - Assembly parser and compiler

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
