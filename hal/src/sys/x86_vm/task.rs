use x86_64::registers::model_specific as msr;

#[repr(C)]
#[derive(Default)]
pub struct UserCtx {
    /// The "foreign" stack pointer (i.e. user sp in kernel context, and
    /// kernel sp in user context).
    ///
    /// This is read and written using a `xchg gs:[0], rsp` instruction.
    pub foreign_sp: usize,

    // used by assembly code, should not be touched by Rust code
    prev_kctx: usize,
    cur_kctx: usize,
}

#[repr(C)]
#[derive(Default)]
pub struct KernelCtx {
    pub rsp: usize,
    pub rbp: usize,
    pub rbx: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub gs_offset: usize,
}

const KERNEL_STACK_LAYOUT: alloc::alloc::Layout =
    unsafe { alloc::alloc::Layout::from_size_align_unchecked(0x4000, 0x1000) };

impl UserCtx {
    pub fn from_user_sp(user_sp: usize) -> UserCtx {
        UserCtx {
            foreign_sp: user_sp,
            ..Default::default()
        }
    }
}

impl KernelCtx {
    pub fn allocate_for(thread_ctx: *const UserCtx) -> KernelCtx {
        let stack = unsafe { alloc::alloc::alloc(KERNEL_STACK_LAYOUT) };
        // write the task's entry address at the bottom of the stack
        unsafe {
            (stack.offset(0x3FF8) as *mut u64).write(user_entry as u64);
        }
        KernelCtx {
            rsp: (stack as usize) + 0x3FF8,
            gs_offset: thread_ctx as usize,
            ..Default::default()
        }
    }
}

extern "C" {
    // functions defined in `task.asm`
    pub fn ktask_enter(from: *mut KernelCtx, to: *mut KernelCtx);
    pub fn ktask_leave();
    // functions defined in `x86-vm-kernel`
    fn user_entry() -> !;
}

global_asm!(include_str!("task.asm"));

pub fn ensure_ktask_context() {
    // IA32_KERNEL_GS_BASE must be 0 in a kernel context
    assert_eq!(msr::KernelGsBase::read().as_u64(), 0);
    // the opposite for IA32_GS_BASE
    assert_ne!(msr::GsBase::read().as_u64(), 0);
}
