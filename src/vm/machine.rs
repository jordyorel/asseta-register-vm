use std::time::Instant;
use super::instruction::InstructionSet;

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

    fn fetch(&self) -> i32 {
        self.program[self.pc]
    }

    fn execute(&mut self) {
        if self.pc >= MAX_PROGRAM_SIZE {
            eprintln!("Error: Program counter out of bounds: {}", self.pc);
            self.running = false;
            return;
        }
        let instruction = self.fetch();
        self.pc += 1;
        self.instruction_count += 1; // Increment instruction count

        println!("PC: {}, Registers: {:?}", self.pc, self.registers);

        match instruction {
            i if i == InstructionSet::Halt as i32 => {
                println!("HALT instruction encountered. Shutting down VM.");
                self.running = false;
            }
            i if i == InstructionSet::LoadConst as i32 => {
                if self.pc + 1 >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for LOAD_CONST");
                    self.running = false;
                    return;
                }
                let reg_idx = self.program[self.pc] as usize;
                self.pc += 1;
                let value = self.program[self.pc];
                self.pc += 1;
                println!("LOAD_CONST R{}, {}", reg_idx, value);
                if reg_idx < NUM_REGISTERS {
                    self.registers[reg_idx] = value;
                } else {
                    eprintln!("Error: Invalid register index {}", reg_idx);
                    self.running = false;
                }
            }
            i if i == InstructionSet::Mov as i32 => {
                if self.pc + 1 >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for MOV");
                    self.running = false;
                    return;
                }
                let des_reg = self.program[self.pc] as usize;
                self.pc += 1;
                let src_reg = self.program[self.pc] as usize;
                self.pc += 1;
                println!("MOV R{}, R{}", des_reg, src_reg);
                if des_reg < NUM_REGISTERS && src_reg < NUM_REGISTERS {
                    self.registers[des_reg] = self.registers[src_reg];
                } else {
                    eprintln!("Error: Invalid register index");
                    self.running = false;
                }
            }
            i if i == InstructionSet::Add as i32 => {
                if self.pc + 1 >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for ADD");
                    self.running = false;
                    return;
                }
                let reg1_idx = self.program[self.pc] as usize;
                self.pc +=1;
                let reg2_idx = self.program[self.pc] as usize;
                self.pc += 1;
                println!("ADD R{}, R{}", reg1_idx, reg2_idx);
                if reg1_idx < NUM_REGISTERS && reg2_idx < NUM_REGISTERS {
                    self.registers[reg1_idx] += self.registers[reg2_idx];
                } else {
                    eprintln!("Error: Invalid register index");
                    self.running = false;
                }
            }
            i if i == InstructionSet::Sub as i32 => {
                if self.pc + 1 >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for SUB");
                    self.running = false;
                    return;
                }
                let reg1_idx = self.program[self.pc] as usize;
                self.pc += 1;
                let reg2_idx = self.program[self.pc] as usize;
                self.pc += 1;
                println!("SUB R{}, R{}", reg1_idx, reg2_idx);
                if reg1_idx < NUM_REGISTERS && reg2_idx < NUM_REGISTERS {
                    self.registers[reg1_idx] -= self.registers[reg2_idx];
                } else {
                    eprintln!("Error: Invalid register index");
                    self.running = false;
                }
            }
            i if i == InstructionSet::Mul as i32 => {
                if self.pc + 1 >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for MUL");
                    self.running = false;
                    return;
                }
                let reg1_idx = self.program[self.pc] as usize;
                self.pc += 1;
                let reg2_idx = self.program[self.pc] as usize;
                self.pc += 1;
                println!("MUL R{}, R{}", reg1_idx, reg2_idx);
                if reg1_idx < NUM_REGISTERS && reg2_idx < NUM_REGISTERS {
                    self.registers[reg1_idx] *= self.registers[reg2_idx];
                } else {
                    eprintln!("Error: Invalid register index");
                    self.running = false;
                }
            }
            i if i == InstructionSet::Mod as i32 => {
                if self.pc + 1 >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for MOD");
                    self.running = false;
                    return;
                }
                let reg1_idx = self.program[self.pc] as usize;
                self.pc += 1;
                let reg2_idx = self.program[self.pc] as usize;
                self.pc += 1;
                println!("MOD R{}, R{}", reg1_idx, reg2_idx);
                if reg1_idx < NUM_REGISTERS && reg2_idx < NUM_REGISTERS {
                    self.registers[reg1_idx] %= self.registers[reg2_idx];
                } else {
                    eprintln!("Error: Invalid register index");
                    self.running = false;
                }
            }
            i if i == InstructionSet::Div as i32 => {
                if self.pc + 1 >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for DIV");
                    self.running = false;
                    return;
                }
                let reg1_idx = self.program[self.pc] as usize;
                self.pc += 1;
                let reg2_idx = self.program[self.pc] as usize;
                self.pc += 1;
                println!("DIV R{}, R{}", reg1_idx, reg2_idx);
                if reg1_idx < NUM_REGISTERS && reg2_idx < NUM_REGISTERS {
                    self.registers[reg1_idx] /= self.registers[reg2_idx];
                } else {
                    eprintln!("Error: Invalid register index");
                    self.running = false;
                }
            }
            i if i == InstructionSet::PrintReg as i32 => {
                if self.pc >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for PRINT_REG");
                    self.running = false;
                    return;
                }
                let reg_idx = self.program[self.pc] as usize;
                self.pc += 1;
                println!("PRINT_REG, R{}", reg_idx);
                if reg_idx < NUM_REGISTERS {
                    println!("Register R{} = {}", reg_idx,  self.registers[reg_idx]);
                } else {
                    eprintln!("Error: Invalid register index {}", reg_idx);
                    self.running = false;
                }
            }
            i if i == InstructionSet::Jmp as i32 => {
                if self.pc >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for JMP");
                    self.running = false;
                    return;
                }
                let addr = self.program[self.pc] as usize;
                self.pc += 1;
                println!("JMP {}", addr);
                if addr < MAX_PROGRAM_SIZE {
                    self.pc = addr;
                } else {
                    eprintln!("Error: Invalid jump address {}", addr);
                    self.running = false;
                }
            }
            i if i == InstructionSet::JmpIfNotZero as i32 => {
                if self.pc + 1 >= MAX_PROGRAM_SIZE {
                    eprintln!("Error: Not enough operands for JMP_IF_NOT_ZERO");
                    self.running = false;
                    return;
                }
                let reg_idx = self.program[self.pc] as usize;
                self.pc += 1;
                let addr = self.program[self.pc] as usize;
                self.pc += 1;
                println!("JMP_IF_NOT_ZERO R{}, {}", reg_idx, addr);
                if reg_idx < NUM_REGISTERS {
                    if self.registers[reg_idx] != 0 {
                        if addr < MAX_PROGRAM_SIZE {
                            self.pc = addr;
                        } else {
                            eprintln!("Error: Invalid jump address {}", addr);
                            self.running = false;
                        }
                    }
                } else {
                    eprintln!("Error: Invalid register index {} in JMP_IF_NOT_ZERO", reg_idx);
                    self.running = false;
                }
            }
            _=> {
                eprintln!("Error: Unknown instruction {} at PC={}", instruction, self.pc - 1);
                self.running = false;
            }
        }
    }

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
