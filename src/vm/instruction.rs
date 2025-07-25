/// Define the instruction set for the virtual machine
/// Each instruction is represented by an enum variant with a unique value
/// The values are used to identify the instruction in the program memory
/// The enum is also used to provide a more readable representation of the instructions
/// The values are chosen to be sequential integers starting from 0
/// This allows for easy conversion between the enum and the integer representation
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
    Jmp = 9,                // JMP <addr>
    JmpIfNotZero = 10,      // JMP_IF_NOT_ZERO <reg> <addr>
}
