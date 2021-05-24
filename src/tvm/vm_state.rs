use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct VMState {
    stack       : Vec<u64>,
    call_stack  : Vec<u64>
}

impl VMState {
    pub fn from(state : VMState) -> VMState {
        VMState {stack : state.stack, call_stack : state.call_stack}
    }

    pub fn new() -> VMState {
        VMState {stack : Vec::new(), call_stack : Vec::new()}
    }

    pub fn push(&mut self, data : u64) {
        self.stack.push(data);
    }

    pub fn pop(&mut self) -> Option<u64> {
        self.stack.pop()
    }
}
