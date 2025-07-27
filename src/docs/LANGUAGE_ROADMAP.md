# Orus Language Evolution Roadmap 🗺️

Based on your current implementation, here's a structured roadmap for evolving your Orus language from basic functionality to a more complete programming language:

## Phase 1: Core Language Foundation (Current → Week 2)
**Priority: Critical** | **Current Status: 70% Complete**

### 1.1 Expression System Enhancement
- **Where to start**: `src/compiler/parser.rs:16-37` (BinaryOp AST node)
- Add comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- Implement operator precedence parsing
- Add parenthesized expressions support
- Add unary operators (`-`, `+`, `!`)

### 1.2 Control Flow Completion
- **Where to start**: `src/compiler/parser.rs:23-33` (ForLoop AST node)
- Fix for loop code generation in `src/compiler/codegen.rs`
- Add `if/else` statements
- Add `while` loops
- Implement proper label/jump management

### 1.3 Variable System Improvements
- **Where to start**: `src/compiler/parser.rs:6-15` (VariableDeclaration)
- Add type annotations (`x: int = 5`)
- Implement variable scoping
- Add const vs mut distinction enforcement

## Phase 2: Type System & Safety (Week 3-4)
**Priority: High** | **Foundation for advanced features**

### 2.1 Basic Type System
- **Where to start**: Create `src/compiler/types.rs`
- Define basic types: `int`, `bool`, `string`, `float`
- Add type checking in parser
- Implement type inference
- Add compile-time type error reporting

### 2.2 Memory Management
- **Where to start**: `src/vm/machine.rs` (expand beyond 4 registers)
- Add stack frame management
- Implement heap allocation for strings/arrays
- Add garbage collection or reference counting

## Phase 3: Functions & Modularity (Week 5-6)
**Priority: High** | **Essential for real programs**

### 3.1 Function System
- **Where to start**: Add `Function` variant to `src/compiler/parser.rs:5-37`
- Function declarations and calls
- Parameter passing and return values
- Local variable scoping
- Recursive function support

### 3.2 Built-in Functions
- **Where to start**: `src/vm/executor.rs` (add new instruction types)
- String operations (`len`, `concat`, `substr`)
- Math functions (`sqrt`, `pow`, `abs`)
- I/O functions (`input`, `read_file`, `write_file`)

## Phase 4: Data Structures (Week 7-8)
**Priority: Medium** | **Enables complex programs**

### 4.1 Arrays/Lists
- **Where to start**: Add `Array` AST node in parser
- Dynamic arrays with indexing
- Array methods (`push`, `pop`, `len`)
- Array slicing (`arr[1..5]`)

### 4.2 Structs/Objects
- **Where to start**: Create `src/compiler/struct_parser.rs`
- Struct definitions and instantiation
- Field access and modification
- Method definitions on structs

## Phase 5: Advanced Features (Week 9-12)
**Priority: Medium** | **Professional language features**

### 5.1 Error Handling
- **Where to start**: Add `Result` and `Option` types
- Try/catch mechanisms
- Panic handling
- Error propagation

### 5.2 Pattern Matching
- **Where to start**: Add `Match` AST node
- Switch/case statements
- Destructuring assignment
- Guards and complex patterns

### 5.3 Iterators & Closures
- **Where to start**: Extend function system
- Anonymous functions/lambdas
- Iterator protocols
- Higher-order functions (`map`, `filter`, `reduce`)

## Phase 6: Standard Library (Week 13-16)
**Priority: Low-Medium** | **Productivity features**

### 6.1 Core Libraries
- **Where to start**: Create `src/stdlib/` directory
- Collections (HashMap, Set, Queue)
- String processing utilities
- Math and random number generation
- Date/time handling

### 6.2 I/O & Networking
- File system operations
- HTTP client functionality
- JSON parsing/serialization
- Command-line argument parsing

## Phase 7: Optimization & Tooling (Week 17+)
**Priority: Low** | **Performance and developer experience**

### 7.1 Performance Optimization
- **Where to start**: `src/vm/machine.rs` optimization
- Bytecode optimization passes
- Just-in-time compilation
- Register allocation improvements
- Memory layout optimization

### 7.2 Developer Tools
- **Where to start**: Create `tools/` directory
- Debugger with breakpoints
- Language server protocol (LSP)
- Package manager
- Documentation generator

## Implementation Strategy

### Quick Wins (Start Here!)
1. **Fix for loops** - `src/compiler/codegen.rs` (addresses current limitation)
2. **Add if/else** - Extend parser AST and codegen
3. **Comparison operators** - Minimal VM instruction additions
4. **String literals** - Add to lexer and basic string support

### Architecture Decisions
- **VM Instructions**: Expand from 11 to ~30 instructions gradually
- **Memory Model**: Move from 4 registers to register + stack architecture
- **Error Handling**: Add comprehensive error types early
- **Testing**: Add test suite at each phase

### Development Order Rationale
1. **Core language first** - Ensures solid foundation
2. **Types before functions** - Prevents major refactoring later
3. **Functions before data structures** - Natural dependency order
4. **Standard library last** - Can be developed in parallel

This roadmap prioritizes practical functionality while maintaining architectural coherence. Each phase builds naturally on the previous one, allowing for incremental development and testing.

## Current Architecture Overview

### Existing Components
- **Lexer** (`src/compiler/lexer.rs`): Tokenizes source code with indent/dedent support
- **Parser** (`src/compiler/parser.rs`): Generates AST from tokens
- **Code Generator** (`src/compiler/codegen.rs`): Converts AST to VM assembly
- **VM** (`src/vm/machine.rs`): 4-register virtual machine with program counter
- **Instruction Set** (`src/vm/instruction.rs`): 11 instruction types
- **Executor** (`src/vm/executor.rs`): Individual instruction implementations
- **Assembler** (`src/vm/assembler.rs`): Two-pass assembler for bytecode generation

### Current Capabilities
✅ Variable assignment and arithmetic  
✅ Basic print statements  
✅ Simple expression evaluation  
✅ Assembly generation and execution  
⚠️ Limited for loop support  
❌ No functions, types, or control flow  

### Next Immediate Steps
1. Fix for loop implementation in codegen
2. Add if/else conditional statements
3. Implement comparison operators
4. Add string literal support