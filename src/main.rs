mod compiler;
mod vm;

use std::env;
use std::fs;
use compiler::{lexer::Lexer, parser::Parser, codegen::CodeGenerator};
use vm::{assembler::assemble, machine::VM};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let source = if args.len() > 1 {
        let path = &args[1];
        fs::read_to_string(path).expect("Failed to read .orus file")
    } else {
        // Default program - simple without indentation issues
        r#"mut sum = 5
print(sum)"#.to_string()
    };

    // Tokenize
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    
    // Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    
    // Generate code
    let mut codegen = CodeGenerator::new();
    let asm = codegen.generate(&ast);
    println!("Generated Assembly:\n{}", asm);
    
    // Add HALT to end the program
    let asm_with_halt = format!("{}\nHALT", asm);
    
    // Assemble and run
    let program = assemble(&asm_with_halt);
    let mut vm = VM::new();
    vm.load_program(&program);
    vm.run();
}
