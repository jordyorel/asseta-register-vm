/// Define the instruction set for the virtual machine
#[derive(Debug, Clone, Copy)]
pub enum InstructionSet {
    LoadConst = 0,          // LOAD_CONST <reg> <value>
    Mov = 1,                // MOV <dest_reg> <src_reg>
    Add = 2,                // ADD <reg1> <reg2>
    Sub = 3,                // SUB <reg1> <reg2>
    Mul = 4,                // MUL <reg1> <reg2>
    Mod = 5,                // MOD <reg1> <reg2>
    Div = 6,                // DIV <reg1> <reg2>
    PrintReg = 7,           // PRINT_REG <reg>
    Halt = 8,               // HALT
    Jump = 9,                // JMP <addr>
    JumpIfNotZero = 10,      // JMP_IF_NOT_ZERO <reg> <addr>

}
