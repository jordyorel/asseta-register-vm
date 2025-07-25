use super::instruction::InstructionSet;


pub fn assemble(asm: &str) -> Vec<i32> {
    let mut program = Vec::new();
    let mut labels = std::collections::HashMap::new();
    let mut current_addr = 0;

    // First pass: Identify labels and their addresses
    for line in asm.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if line.ends_with(':') {
            let label = line.trim_end_matches(':').trim();
            labels.insert(label.to_string(), current_addr);
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "LOAD_CONST" => current_addr += 3,
            "ADD" | "SUB" | "MUL" | "DIV" | "MOD" | "MOV" | "JMP_IF_NOT_ZERO" => current_addr += 3,
            "PRINT_REG" => current_addr += 2,
            "JMP" => current_addr += 2,
            "HALT" => current_addr += 1,
            _ => panic!("Unknown instruction: {}", parts[0]),
        }
    }

    // Second pass: Generate program
    for line in asm.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") || line.ends_with(':') {
            continue;
        }
        let parts: Vec<&str> = line.split(|c| c == ' ' || c == ',')
            .filter(|p| !p.is_empty())
            .collect();

        match parts[0] {
            "LOAD_CONST" => {
                let reg = parts[1].trim_start_matches("R").parse::<i32>().unwrap();
                let value = parts[2].parse::<i32>().unwrap();
                program.extend([InstructionSet::LoadConst as i32, reg, value]);
            }
            "ADD" => {
                let reg1 = parts[1].trim_start_matches("R").parse::<i32>().unwrap();
                let reg2 = parts[2].trim_start_matches("R").parse::<i32>().unwrap();
                program.extend([InstructionSet::Add as i32, reg1, reg2]);
            }
            "SUB" => {
                let reg1 = parts[1].trim_start_matches("R").parse::<i32>().unwrap();
                let reg2 = parts[2].trim_start_matches("R").parse::<i32>().unwrap();
                program.extend([InstructionSet::Sub as i32, reg1, reg2]);
            }
            "MUL" => {
                let reg1 = parts[1].trim_start_matches("R").parse::<i32>().unwrap();
                let reg2 = parts[2].trim_start_matches("R").parse::<i32>().unwrap();
                program.extend([InstructionSet::Mul as i32, reg1, reg2]);
            }
            "JMP_IF_NOT_ZERO" => {
                let reg = parts[1].trim_start_matches("R").parse::<i32>().unwrap();
                let label = parts[2];
                let addr = *labels.get(label).expect("Unknown label") as i32;
                program.extend([InstructionSet::JmpIfNotZero as i32, reg, addr]);
            }
            "PRINT_REG" => {
                let reg = parts[1].trim_start_matches("R").parse::<i32>().unwrap();
                program.extend([InstructionSet::PrintReg as i32, reg]);
            }
            "HALT" => {
                program.push(InstructionSet::Halt as i32);
            }
            _ => panic!("Unknown instruction: {}", parts[0]),
        }
    }
    program
}
