use std::time::Instant;
use super::instruction::InstructionSet;
use super::executor;

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
            registers: [0; NUM_REGISTERS],    // Initialize register to 0
            pc: 0,                            // Start at instruction 0
            program: [0; MAX_PROGRAM_SIZE],   // Initialize program memory to 0
            running: true,                    // VM starts in running state
            instruction_count: 0,             // Initialize to 0
        }
    }

    // Helper function to fetch register index with bounds checking
    pub fn get_register(&mut self) -> Option<usize> {
        if self.pc >= MAX_PROGRAM_SIZE {
            eprintln!("Error: Not enough operands for instruction at PC={}", self.pc);
            self.running = false;
            return None;
        }
        let reg_idx = self.program[self.pc] as usize;
        self.pc += 1; // Move to the next instruction
        
        if reg_idx >= NUM_REGISTERS {
            eprintln!("Error: Invalid register index: {}", reg_idx);
            self.running = false;
            return None;
        }
        Some(reg_idx)
    }

    // Helper function to fetch immediate values with bounds checking
    pub fn get_immediate(&mut self) -> Option<i32> {
        if self.pc >= MAX_PROGRAM_SIZE {
            eprintln!("Error: Not enough operands for instruction at PC={}", self.pc);
            self.running = false;
            return None;
        }
        let value = self.program[self.pc];
        self.pc += 1; // Move to the next instruction
        Some(value)
    }


    // Load a program into the VM
    pub fn load_program(&mut self, prog: &[i32]) {
        if prog.len() > MAX_PROGRAM_SIZE {
            eprintln!("Error: Program size {} exceeds maximum memory {}", prog.len(), MAX_PROGRAM_SIZE);
            self.running = false;
            return;
        }
        for (i, &instruction) in prog.iter().enumerate() {
            self.program[i] = instruction
        }         
    }

    /// Fetch the next instruction from the program memory
    fn fetch(&self) -> i32 {
        self.program[self.pc]
    }

    /// Execute the current instruction
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

        println!("PC: {}, Registers: {:?}", self.pc, self.registers);

        // Call the executor function directly
        executor::execute_instruction(self, instruction);
    }


    /// Run the virtual machine
    pub fn run(&mut self) {
        println!("--- VM Start ---");
        let start = Instant::now(); // Start timing
        while self.running && self.pc < MAX_PROGRAM_SIZE {
            self.execute();
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
