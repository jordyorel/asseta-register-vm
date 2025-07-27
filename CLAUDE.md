# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a register-based virtual machine written in Rust that executes custom assembly programs. The project consists of two main components:

1. **Compiler Pipeline**: Lexer → Parser → Code Generator that compiles a high-level language (.orus files) to VM assembly
2. **Virtual Machine**: Assembler + Executor that runs the generated assembly on a register-based VM

## Architecture

### Core Components

- **VM Module** (`src/vm/`): Contains the virtual machine implementation
  - `machine.rs`: VM struct with 4 registers (R0-R3), program counter, and execution loop
  - `instruction.rs`: Instruction set enum with 11 instruction types
  - `executor.rs`: Individual instruction execution functions
  - `assembler.rs`: Two-pass assembler that converts assembly text to bytecode

- **Compiler Module** (`src/compiler/`): Compiles high-level language to assembly
  - `lexer.rs`: Tokenizer for the source language
  - `parser.rs`: AST generator
  - `codegen.rs`: Assembly code generator

### Execution Flow

1. Source code (.orus) → Lexer → Parser → AST → CodeGenerator → Assembly
2. Assembly → Assembler → Bytecode → VM execution

## Common Commands

### Build and Run
```bash
cargo build          # Build the project
cargo run [file]      # Run VM with optional .orus file (defaults to built-in program)
cargo check           # Check compilation without building
```

### Testing
```bash
cargo test            # Run tests (if any exist)
```

### Example Usage
```bash
cargo run             # Run VM with test assembly program (sum 1+2+...+10 = 55)
# Note: Compiler pipeline (lexer/parser/codegen) needs work for .orus files
```

## Current Status

✅ **All VM compilation errors fixed** - Added missing `get_register()` and `get_immediate()` methods to the VM struct and updated the execute method to use the executor functions properly.

✅ **VM and Executor working correctly** - The machine.rs and executor.rs files now work together properly. VM successfully executes assembly programs and produces correct results.

✅ **Fixed architecture** - Updated sample.orus to contain proper high-level syntax and main.rs to handle compilation pipeline.

⚠️ **Compiler Issues**: The lexer/parser/codegen pipeline has stack overflow issues and needs implementation work. Currently main.rs is set to test VM with assembly only.

## VM Instruction Set

The VM supports 11 instructions:
- `LOAD_CONST`: Load immediate value into register
- `MOV`: Copy register to register  
- `ADD/SUB/MUL/DIV/MOD`: Arithmetic operations
- `PRINT_REG`: Print register value
- `HALT`: Stop execution
- `JMP`: Unconditional jump
- `JMP_IF_NOT_ZERO`: Conditional jump

## Memory Layout

- 4 general-purpose registers (R0-R3)
- Program memory: 256 instructions max
- Stack size: 4MB (configured in main.rs)

## File Extensions

- `.orus`: Contains high-level source code (as updated in `sample.orus`)
- Assembly is generated as intermediate representation during compilation

## Implementation Status

✅ **Working Components:**
- VM core (machine.rs) - register-based execution
- Executor (executor.rs) - individual instruction implementations  
- Assembler - converts assembly text to bytecode
- Instruction set - 11 instructions fully implemented

⚠️ **Needs Work:**
- Compiler pipeline (lexer/parser/codegen) - has stack overflow issues
- High-level language compilation to assembly

## Next Steps for Development

1. Fix the lexer/parser/codegen stack overflow issues
2. Implement proper AST generation and code generation
3. Add more high-level language features (variables, control flow, functions)
4. Add error handling and better diagnostics