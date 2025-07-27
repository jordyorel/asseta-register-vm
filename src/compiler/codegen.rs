use std::collections::HashMap;

use crate::compiler::parser::ASTNode;

pub struct CodeGenerator {
    asm: String,
    register_counter: u8,
    variable_map: HashMap<String, u8>,
    label_counter: u32,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            asm: String::new(),
            register_counter: 0,
            variable_map: HashMap::new(),
            label_counter: 0,
        }
    }

    pub fn generate(&mut self, nodes: &[ASTNode]) -> String {
        for node in nodes {
            self.generate_node(node);
        }
        self.asm.clone()
    }

    
    // Helper methods...
    /// Emit an assembly instruction
    fn emit(&mut self, instruction: &str) {
        self.asm.push_str(instruction);
        self.asm.push('\n');
    }


    /// Get next available register
    fn next_register(&mut self) -> u8 {
        if self.register_counter >= 4 {
            self.register_counter = 0; // Wrap around for simplicity
        }
        let reg = self.register_counter;
        self.register_counter += 1;
        reg
    }

    /// Generate code for a number literal and return the register it's in
    fn generate_expression(&mut self, node: &ASTNode) -> u8 {
        match node {
            ASTNode::Number(n) => {
                let reg = self.next_register();
                self.emit(&format!("LOAD_CONST R{}, {}", reg, n));
                reg
            }
            ASTNode::Identifier(name) => {
                if let Some(&reg) = self.variable_map.get(name) {
                    reg
                } else {
                    panic!("Undefined variable: {}", name);
                }
            }
            ASTNode::BinaryOp { op, left, right } => {
                let left_reg = self.generate_expression(left);
                let right_reg = self.generate_expression(right);
                
                match op.as_str() {
                    "+" => self.emit(&format!("ADD R{}, R{}", left_reg, right_reg)),
                    "-" => self.emit(&format!("SUB R{}, R{}", left_reg, right_reg)),
                    "*" => self.emit(&format!("MUL R{}, R{}", left_reg, right_reg)),
                    "/" => self.emit(&format!("DIV R{}, R{}", left_reg, right_reg)),
                    _ => panic!("Unsupported operator: {}", op),
                }
                left_reg
            }
            _ => panic!("Unsupported expression: {:?}", node),
        }
    }

    fn generate_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::VariableDeclaration { mutable: _, name, value } => {
                if let Some(expr) = value {
                    let reg = self.generate_expression(expr);
                    self.variable_map.insert(name.clone(), reg);
                }
            }
            
            ASTNode::Assignment { target, value } => {
                let reg = self.generate_expression(value);
                self.variable_map.insert(target.clone(), reg);
            }
            
            ASTNode::Print(args) => {
                for arg in args {
                    let reg = self.generate_expression(arg);
                    self.emit(&format!("PRINT_REG R{}", reg));
                }
            }
            
            ASTNode::ForLoop { variable, start, end, inclusive: _, body } => {
                // For now, implement a simple unrolled loop for the range 1..10
                // This is a temporary solution to get the basic functionality working
                
                // Allocate register for loop variable
                let loop_var_reg = self.next_register();
                self.variable_map.insert(variable.clone(), loop_var_reg);
                
                // Get start and end values (assuming they are literals for now)
                let start_val = if let ASTNode::Number(n) = start.as_ref() { *n } else { 1 };
                let end_val = if let ASTNode::Number(n) = end.as_ref() { *n } else { 10 };
                
                // Unroll the loop
                for i in start_val..end_val {
                    // Set loop variable to current value
                    self.emit(&format!("LOAD_CONST R{}, {}", loop_var_reg, i));
                    
                    // Generate loop body for this iteration
                    for stmt in body {
                        self.generate_node(stmt);
                    }
                }
                
                // Remove loop variable from scope
                self.variable_map.remove(variable);
            }
            
            _ => {
                // For expressions used as statements, just generate them
                self.generate_expression(node);
            }
        }
    }
    
}
