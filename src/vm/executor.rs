use super::machine::{VM, MAX_PROGRAM_SIZE, NUM_REGISTERS};
use super::instruction::InstructionSet;

pub fn execute_instruction(vm: &mut VM, instruction: InstructionSet) {
    match instruction {
        InstructionSet::LoadConst => execute_load_const(vm),
        InstructionSet::Add => execute_add(vm),
        InstructionSet::Sub => execute_sub(vm),
        InstructionSet::Mul => execute_mul(vm),
        InstructionSet::Div => execute_div(vm),
        InstructionSet::Mod => execute_mod(vm),
        InstructionSet::Mov => execute_mov(vm),
        InstructionSet::PrintReg => execute_print_reg(vm),
        InstructionSet::Halt => execute_halt(vm),
        InstructionSet::Jump => execute_jump(vm),
        InstructionSet::JumpIfNotZero => execute_jump_if_not_zero(vm),
    }
}
fn execute_load_const(vm: &mut VM) {
    let reg_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };
    let value = match vm.get_immediate() {
        Some(val) => val,
        None => return,
    };

    println!("LOAD_CONST R{}, {}", reg_idx, value);

    if reg_idx >= NUM_REGISTERS {
        eprintln!("Error: Invalid register index {}", reg_idx);
        vm.running = false;
        return;
    }

    vm.registers[reg_idx] = value;
}

fn execute_add(vm: &mut VM) {
    let reg1_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };
    let reg2_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };

    println!("ADD R{}, R{}", reg1_idx, reg2_idx);

    if reg1_idx >= NUM_REGISTERS || reg2_idx >= NUM_REGISTERS {
        eprintln!("Error: Invalid register index");
        vm.running = false;
        return;
    }

    vm.registers[reg1_idx] += vm.registers[reg2_idx];
}

fn execute_sub(vm: &mut VM) {
    let reg1_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };
    let reg2_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };

    println!("SUB R{}, R{}", reg1_idx, reg2_idx);

    if reg1_idx >= NUM_REGISTERS || reg2_idx >= NUM_REGISTERS {
        eprintln!("Error: Invalid register index");
        vm.running = false;
        return;
    }

    vm.registers[reg1_idx] -= vm.registers[reg2_idx];
}

fn execute_mul(vm: &mut VM) {
    let reg1_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };
    let reg2_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };

    println!("MUL R{}, R{}", reg1_idx, reg2_idx);

    if reg2_idx >= NUM_REGISTERS || reg1_idx >= NUM_REGISTERS {
        eprintln!("Error: Invalid register index");
        vm.running = false;
        return;
    }

    vm.registers[reg1_idx] = vm.registers[reg1_idx].wrapping_mul(vm.registers[reg2_idx]);
}

fn execute_div(vm: &mut VM) {
    let reg1_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };
    let reg2_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };

    println!("DIV R{}, R{}", reg1_idx, reg2_idx);

    if reg1_idx >= NUM_REGISTERS || reg2_idx >= NUM_REGISTERS {
        eprintln!("Error: Invalid register index");
        vm.running = false;
        return;
    }

    if vm.registers[reg2_idx] == 0 {
        eprintln!("Error: Division by zero");
        vm.running = false;
        return;
    }

    vm.registers[reg1_idx] /= vm.registers[reg2_idx];
}

fn execute_mod(vm: &mut VM) {
    let reg1_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };
    let reg2_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };

    println!("MOD R{}, R{}", reg1_idx, reg2_idx);

    if reg1_idx >= NUM_REGISTERS || reg2_idx >= NUM_REGISTERS {
        eprintln!("Error: Invalid register index");
        vm.running = false;
        return;
    }

    if vm.registers[reg2_idx] == 0 {
        eprintln!("Error: Modulo by zero");
        vm.running = false;
        return;
    }

    vm.registers[reg1_idx] %= vm.registers[reg2_idx];
}

fn execute_mov(vm: &mut VM) {
    let dest_reg = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };
    let src_reg = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };

    println!("MOV R{}, R{}", dest_reg, src_reg);

    if dest_reg >= NUM_REGISTERS || src_reg >= NUM_REGISTERS {
        eprintln!("Error: Invalid register index");
        vm.running = false;
        return;
    }

    vm.registers[dest_reg] = vm.registers[src_reg];
}

fn execute_print_reg(vm: &mut VM) {
    let reg_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };

    println!("PRINT_REG R{}", reg_idx);

    if reg_idx >= NUM_REGISTERS {
        eprintln!("Error: Invalid register index");
        vm.running = false;
        return;
    }

    println!("Register R{} = {}", reg_idx,  vm.registers[reg_idx]);
}

fn execute_jump(vm: &mut VM) {
    let addr = match vm.get_immediate() {
        Some(addr) => addr as usize,
        None => return,
    };

    println!("JMP to address {}", addr);

    if addr >= MAX_PROGRAM_SIZE {
        eprintln!("Error: Jump address out of bounds");
        vm.running = false;
        return;
    }

    vm.pc = addr;
}

fn execute_jump_if_not_zero(vm: &mut VM) {
    let reg_idx = match vm.get_register() {
        Some(reg) => reg,
        None => return,
    };
    let addr = match vm.get_immediate() {
        Some(addr) => addr as usize,
        None => return,
    };

    println!("JUMP_IF_NOT_ZERO R{}, {}", reg_idx, addr);

    if reg_idx >= NUM_REGISTERS {
        eprintln!("Error: Invalid register index");
        vm.running = false;
        return;
    }

    if vm.registers[reg_idx] != 0 {
        vm.pc = addr;
    }
}

fn execute_halt(vm: &mut VM) {
    vm.running = false;
    println!("HALT instruction encountered. Shutting down VM.");
}
