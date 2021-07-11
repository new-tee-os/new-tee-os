use alloc::boxed::Box;
use alloc::sync::Arc;
use core::future::Future;
use core::mem::MaybeUninit;
use spin::Mutex;

mod pid_pool;

use crate::sys::task::*;
pub use pid_pool::PidPool;

pub type Pid = u32;

pub static PID_POOL: Mutex<PidPool> = Mutex::new(PidPool::new());

pub struct Task {
    pub pid: Pid,

    /// Stores the kernel's `sp` and user's `sp`, and acts as a bridge between
    /// the user context and the kernel context.
    ///
    /// This will be loaded into a register accessible in ISRs, and will be used
    /// to fetch the kernel's stack pointer (`sp`) at the beginning of the ISR.
    /// Therefore, its address must be kept static during the kernel's lifetime.
    /// We use a `Box` to achieve this.
    pub user_ctx: Box<UserCtx>,

    /// Stores the task's kernel context (just before `ktask_leave`).
    /// The task's kernel sp and any callee-saved registers will be put here.
    ///
    /// A `None` indicates a borrowed (vacant) state. When the scheduler switches
    /// to this task, the scheduler takes the `KernelCtx` away (replacing it with
    /// `None`), and then returns it when the task yields back.
    /// This field should never be `None` at any other time.
    pub kernel_ctx: Option<KernelCtx>,
}

impl Task {
    pub fn create(user_sp: usize) -> Task {
        let pid = PID_POOL.try_lock().unwrap().alloc();
        let user_ctx = Box::new(UserCtx::from_user_sp(user_sp));
        // TODO: free kernel stack
        let kernel_ctx = Some(KernelCtx::allocate_for(user_ctx.as_ref()));
        let task = Task {
            pid,
            user_ctx,
            kernel_ctx,
        };
        task
    }
}

pub struct TaskFuture {
    task: Arc<Mutex<Task>>,
}

impl TaskFuture {
    pub fn new(task: Arc<Mutex<Task>>) -> TaskFuture {
        TaskFuture { task }
    }
}

impl Future for TaskFuture {
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        // the scheduler's KernelCtx
        let mut prev_ctx = MaybeUninit::uninit();

        // borrow the Task's KernelCtx
        let next_ctx = self.task.try_lock().unwrap().kernel_ctx.take();
        let mut next_ctx = next_ctx.expect("kernel context is vacant");

        // enter the Task!
        unsafe {
            ktask_enter(prev_ctx.as_mut_ptr(), &mut next_ctx);
        }

        // return the Task's KernelCtx
        self.task.try_lock().unwrap().kernel_ctx.replace(next_ctx);

        cx.waker().wake_by_ref();
        core::task::Poll::Pending
    }
}

pub fn yield_to_sched() {
    ensure_ktask_context();

    unsafe {
        ktask_leave();
    }
}
