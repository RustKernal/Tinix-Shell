pub mod chunk;
pub mod vm_state;
mod instruction_strategies;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    NOP = 0x00,
    PSH = 0x10, POP, DUP, SWP,
    ADD = 0x20, SUB, MUL, DIV, AND, OR , XOR,  NOT,  BSR, BSL, CMP, 
    JMP = 0x30, JMZ, JNZ, JLT, JGT, JEQ, CALL, NTFN,  
    SLP = 0xF0,
    HLT = 0xFF
}

