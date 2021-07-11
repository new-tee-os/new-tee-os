#[repr(C)]
#[derive(Default)]
pub struct UserCtx {
    user_sp: usize,
    kernel_sp: usize,
    // used by assembly code, should not be touched by Rust code
    prev_kctx: usize,
    cur_kctx: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct KernelCtx {
    sp: usize,
    ra: usize,
    tp: usize,
    s0: usize,
    s1: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
}

const KERNEL_STACK_LAYOUT: alloc::alloc::Layout =
    unsafe { alloc::alloc::Layout::from_size_align_unchecked(0x4000, 0x1000) };

impl UserCtx {
    pub fn from_user_sp(user_sp: usize) -> UserCtx {
        UserCtx {
            user_sp,
            ..Default::default()
        }
    }
}

impl KernelCtx {
    pub fn allocate_for(thread_ctx: *const UserCtx) -> KernelCtx {
        let kernel_stack = unsafe { alloc::alloc::alloc(KERNEL_STACK_LAYOUT) };
        assert!(!kernel_stack.is_null(), "failed to allocate kernel stack");
        KernelCtx {
            sp: kernel_stack as usize,
            ra: task_entry as usize,
            tp: thread_ctx as usize,
            ..Default::default()
        }
    }
}

// functions defined in task.S
extern "C" {
    pub fn ktask_enter(from: *mut KernelCtx, to: *mut KernelCtx);
    pub fn ktask_leave();
    fn task_entry() -> !;
}

pub fn ensure_ktask_context() {
    // check if tp is non-zero
    let tp: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) tp);
    }
    assert_ne!(tp, 0);
}

global_asm!(include_str!("./task.S"));
