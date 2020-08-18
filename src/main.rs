#![feature(llvm_asm)]
#![feature(naked_functions)]

use std::ptr;
const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2; // 2MB
const MAX_THREADS: usize = 4;
static RUNTIME: usize = 0;
pub struct Runtime {
    threads: Vec<Thread>,
    current: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Available,
    Running,
    Ready,
}

struct Thread {
    id: usize,
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State,
}

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

fn main() {
    start_stack_switch();
}

const STACK_SIZE: isize = 1024;
fn start_stack_switch() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; STACK_SIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(STACK_SIZE);
        let stack_bottom_aligned = (stack_bottom as usize & !15) as *mut u8;
        std::ptr::write(stack_bottom_aligned.offset(-16) as *mut u64, message as u64);
        ctx.rsp = stack_bottom_aligned.offset(-16) as u64;
        stack_switch(&mut ctx);
    }
}

fn message() {
    let s = "another stack";
    let s1 = String::from(s);
    println!("{}", s1);
    loop {}
}

unsafe fn stack_switch(new: *const ThreadContext) {
    llvm_asm!("
        mov 0x00($0), %rsp
        ret
    "
        :               // output
        : "r"(new)      // input
        :               // clobber list
        : "alignstack"  // options
    );
}
