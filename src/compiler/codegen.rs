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

    /// Emit a label definition
    fn emit_label(&mut self, label: &str) {
        self.asm.push_str(label);
        self.asm.push_str(":\n");
    }

    /// Generate a new unique label
    fn new_label(&mut self) -> String {
        let label = format!("label_{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Allocate a new register
    fn allocate_register(&mut self) -> u8 {
        // We have 256 registers (0-255)
        if self.register_counter >= 255 {
            panic!("Out of registers!");
        }
        let reg = self.register_counter;
        self.register_counter += 1;
        reg
    }

    /// Assign a value to a variable
    fn assign_variable(&mut self, name: &str) {
        let value_reg = self.register_counter;
        let target_reg = if let Some(&reg) = self.variable_map.get(name) {
            reg
        } else {
            // If variable doesn't exist, create a new register for it
            let new_reg = self.allocate_register();
            self.variable_map.insert(name.to_string(), new_reg);
            new_reg
        };

        self.emit(&format!("MOV R{}, R{}", target_reg, value_reg));
    }

    /// Generate code for a number literal
    fn generate_number(&mut self, value: i32) {
        let reg = self.allocate_register();
        self.emit(&format!("LOAD_CONST R{}, {}", reg, value));
        self.register_counter = reg;
    }

    /// Generate code for an identifier
    fn generate_identifier(&mut self, name: &str) {
        if let Some(&existing_reg) = self.variable_map.get(name) {
            // Copy the value to a new register (store value first to avoid borrow conflict)
            let value = existing_reg;
            let new_reg = self.allocate_register();
            self.emit(&format!("MOV R{}, R{}", new_reg, value));
            self.register_counter = new_reg;
        } else {
            panic!("Undefined variable: {}", name);
        }
    }

    // Updated generate_node to handle expressions
    fn generate_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Number(n) => self.generate_number(*n),
            ASTNode::Identifier(name) => self.generate_identifier(name),
            
            ASTNode::VariableDeclaration { mutable: _mutable, name, value } => {
                if let Some(expr) = value {
                    self.generate_node(expr);
                    self.assign_variable(name);
                }
            }
            
            ASTNode::Assignment { target, value } => {
                self.generate_node(value);
                self.assign_variable(target);
            }
            
            ASTNode::ForLoop { variable, start, end, inclusive, body } => {
                // Generate start value
                self.generate_node(start);
                let start_reg = self.register_counter;
                self.emit(&format!("MOV R{}, R{}", start_reg, start_reg));
                
                // Generate end value
                self.generate_node(end);
                let end_reg = self.register_counter;
                self.emit(&format!("MOV R{}, R{}", end_reg, end_reg));
                
                // Allocate register for loop variable
                let loop_var_reg = self.allocate_register();
                self.variable_map.insert(variable.clone(), loop_var_reg);
                self.emit(&format!("MOV R{}, R{}", loop_var_reg, start_reg));
                
                // Create labels
                let loop_start = self.new_label();
                let loop_end = self.new_label();
                
                // Loop header
                self.emit_label(&loop_start);
                
                // Check condition
                let temp_reg = self.allocate_register();
                self.emit(&format!("LOAD_CONST R{}, 0", temp_reg));
                self.emit(&format!("SUB R{}, R{}, R{}", 
                    temp_reg, loop_var_reg, end_reg));
                
                if *inclusive {
                    // For inclusive range (..=), we continue if loop_var <= end
                    self.emit(&format!("JMP_IF_NOT_ZERO R{}, {}", 
                        temp_reg, loop_end));
                } else {
                    // For exclusive range (..), we continue if loop_var < end
                    self.emit(&format!("JMP_IF_NOT_ZERO R{}, {}", 
                        temp_reg, loop_end));
                }
                
                // Generate loop body
                for stmt in body {
                    self.generate_node(stmt);
                }
                
                // Increment loop variable
                self.emit(&format!("LOAD_CONST R{}, 1", temp_reg));
                self.emit(&format!("ADD R{}, R{}, R{}", 
                    loop_var_reg, loop_var_reg, temp_reg));
                
                // Jump back
                self.emit(&format!("JMP {}", loop_start));
                self.emit_label(&loop_end);
            }
            
            ASTNode::Print(args) => {
                for arg in args {
                    self.generate_node(arg);
                    self.emit(&format!("PRINT_REG R{}", self.register_counter));
                }
            }
            
            // Handle other node types...
            _ => panic!("Unsupported AST node: {:?}", node),
        }
    }
    
}
