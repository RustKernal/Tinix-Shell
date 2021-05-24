use super::vm_state;
use alloc::vec::Vec;
use alloc::vec;
pub trait Strategy {
    fn process(&self, state : &mut vm_state::VMState);
}

pub struct NOP;
pub struct PSH(u64);
pub struct POP;



impl Strategy for NOP {
    fn process(&self, _state : &mut vm_state::VMState) {}
}

impl Strategy for PSH {
    fn process(&self, state : &mut vm_state::VMState) {
        state.push(self.0)
    }
}

impl Strategy for POP {
    fn process(&self, state : &mut vm_state::VMState) {
        state.pop();
    }
}


pub struct InstructionTable {
    entries : Vec<&'static dyn Strategy>
}

// impl core::ops::Index<usize> for InstructionTable {
//     type Output = dyn Strategy;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.entries.get_mut(index).unwrap() as Strategy
//     }
// }     

