use super::DEFAULT_STACK_SIZE;

pub const MAX_THREADS: usize = 4;

pub struct Thread {
    pub stack: Vec<u8>,
    pub ctx: ThreadContext,
    pub state: State,
}

#[derive(PartialEq, Eq, Debug)]
pub enum State {
    Available,
    Running,
    Ready, 
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ThreadContext {
    pub rsp: u64,
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbx: u64,
    pub rbp: u64,
}

impl Thread {
    pub fn new() -> Self {
        Thread {
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Available,
        }
    }
}
