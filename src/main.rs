mod vm;

use vm::{assembler::assemble, machine::VM};

fn main() {
    // Test assembly that calculates sum 1+2+3+...+10 = 55
    let test_asm = "
        LOAD_CONST R0, 10
        LOAD_CONST R1, 1
        LOAD_CONST R2, 0
        loopStart:
        ADD R2, R0
        SUB R0, R1
        JMP_IF_NOT_ZERO R0 loopStart
        PRINT_REG R2
        HALT
    ";

    println!("Testing VM with assembly program (sum 1+2+...+10):");
    println!("Expected result: 55\n");
    
    let mut vm = VM::new();
    let program = assemble(test_asm);
    vm.load_program(&program);
    vm.run();
    
    println!("\n=== VM Test Completed ===");
}
