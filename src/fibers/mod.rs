use std::arch::naked_asm;

use runtime::{Runtime, RUNTIME};

pub mod runtime;
pub mod thread;

pub const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;

pub fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
    };
}

#[naked]
pub unsafe extern "C" fn skip() {
    naked_asm!("ret")
}

pub fn yield_thread() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    };
}