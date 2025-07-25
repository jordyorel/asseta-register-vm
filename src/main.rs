mod vm;

use std::env;
use std::fs;
use vm::{assembler::assemble, machine::VM};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Read assembly from file if provided, else use default
    let asm = if args.len() > 1 {
        let path = &args[1];
        // Convert to absolute path if relative
        let abs_path = if path.starts_with('/') {
            path.to_string()
        } else {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            current_dir.join(path).to_string_lossy().to_string()
        };
        fs::read_to_string(&abs_path).expect("Failed to read .orus file")
    } else {
        // Default program if no file is specified
        "
        LOAD_CONST R0, 10
        LOAD_CONST R1, 1
        LOAD_CONST R2, 0
        loopStart:
        ADD R2, R0
        SUB R0, R1
        JMP_IF_NOT_ZERO R0 loopStart
        PRINT_REG R2
        HALT
        ".to_string()
    };

    let mut vm = VM::new();
    let program = assemble(&asm);
    vm.load_program(&program);
    vm.run();
}
