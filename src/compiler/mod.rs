pub mod lexer;
pub mod parser;
pub mod codegen;

pub fn compile(source: &str) -> String {
    // Lexing
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize();
    
    // Parsing
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();
    
    // Code Generation
    let mut codegen = codegen::CodeGenerator::new();
    codegen.generate(&ast)
}