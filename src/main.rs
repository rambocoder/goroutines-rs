#![feature(llvm_asm)]
const STACK_SIZE: isize = 1024;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64
}
fn main() {
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
    println!("ANOTHER STACK");
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