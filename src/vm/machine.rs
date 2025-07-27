use std::time::Instant;
use super::instruction::InstructionSet;
use super::executor::*;

pub const NUM_REGISTERS: usize = 4;
pub const MAX_PROGRAM_SIZE: usize = 256;

// Virtual Machine structure
#[derive(Debug)]
pub struct VM {
    pub registers: [i32; NUM_REGISTERS],      // Array for registers
    pub pc: usize,                            // program counter
    pub program: [i32; MAX_PROGRAM_SIZE],     // program memory
    pub running: bool,                        // Running status
    pub instruction_count: u64,
}

// Implement methods for the InstructionSet enum
impl InstructionSet {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(InstructionSet::LoadConst),
            1 => Some(InstructionSet::Mov),
            2 => Some(InstructionSet::Add),
            3 => Some(InstructionSet::Sub),
            4 => Some(InstructionSet::Mul),
            5 => Some(InstructionSet::Mod),
            6 => Some(InstructionSet::Div),
            7 => Some(InstructionSet::PrintReg),
            8 => Some(InstructionSet::Halt),
            9 => Some(InstructionSet::Jump),
            10 => Some(InstructionSet::JumpIfNotZero),
            _ => None,
        }
    }
}

// Implement methods for the VM
impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; NUM_REGISTERS],
            pc: 0,
            program: [0; MAX_PROGRAM_SIZE],
            running: true,
            instruction_count: 0,
        }
    }

    pub fn load_program(&mut self, prog: &[i32]) {
        if prog.len() > MAX_PROGRAM_SIZE {
            eprintln!("Error: Program size {} exceeds maximum memory {}", prog.len(), MAX_PROGRAM_SIZE);
            self.running = false;
            return;
        }
        self.program[..prog.len()].copy_from_slice(prog);
    }

    pub fn get_register(&mut self) -> Option<usize> {
        if self.pc >= MAX_PROGRAM_SIZE {
            eprintln!("Error: Program counter out of bounds: {}", self.pc);
            self.running = false;
            return None;
        }
        let reg_idx = self.program[self.pc] as usize;
        self.pc += 1;
        Some(reg_idx)
    }

    pub fn get_immediate(&mut self) -> Option<i32> {
        if self.pc >= MAX_PROGRAM_SIZE {
            eprintln!("Error: Program counter out of bounds: {}", self.pc);
            self.running = false;
            return None;
        }
        let value = self.program[self.pc];
        self.pc += 1;
        Some(value)
    }

    fn fetch(&self) -> i32 {
        self.program[self.pc]
    }

    fn execute(&mut self) {
        if self.pc >= MAX_PROGRAM_SIZE {
            eprintln!("Error: Program counter out of bounds: {}", self.pc);
            self.running = false;
            return;
        }

        let instruction = match InstructionSet::from_i32(self.fetch()) {
            Some(i) => i,
            None => {
                eprintln!("Error: Unknown instruction {} at PC={}", self.fetch(), self.pc);
                self.running = false;
                return;
            }
        };
        self.pc += 1;
        self.instruction_count += 1;

        execute_instruction(self, instruction);
    }

    /// Run the virtual machine iteratively to avoid recursion
    pub fn run(&mut self) {
        println!("--- VM Start ---");
        let start = Instant::now(); // Start timing
        
        // Use a loop instead of recursion
        const MAX_ITERATIONS: usize = 1_000_000; // Prevent infinite loops
        
        while self.running && self.pc < MAX_PROGRAM_SIZE && self.instruction_count < MAX_ITERATIONS as u64 {
            self.execute();
        }
        
        if self.instruction_count >= MAX_ITERATIONS as u64 {
            eprintln!("VM stopped: Reached maximum iteration limit");
        }
        
        let elapsed = start.elapsed(); // Get elapsed time
        println!("--- VM End ---");
        println!("Execution time: {:.6} seconds", elapsed.as_secs_f64());
        println!("Total instructions executed: {}", self.instruction_count);
        if elapsed.as_secs_f64() > 0.0 {
            println!(
                "Instructions per second: {:.0}",
                self.instruction_count as f64 / elapsed.as_secs_f64()
            );
        } else {
            println!("Execution time too short to calculate instructions per second");
        }
    }
}