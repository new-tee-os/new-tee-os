use crate::EdgeMemory;

pub trait EdgeCaller: Sync {
    fn acquire(&self);
    fn edge_mem(&self) -> &mut EdgeMemory;
    unsafe fn edge_call(&self);
    fn release(&self);
}

extern "Rust" {
    static GLOBAL_EDGE_CALLER: &'static dyn EdgeCaller;
}

pub fn with_edge_caller<F, V>(f: F) -> V
where
    F: FnOnce(&dyn EdgeCaller) -> V,
{
    let edge_caller = unsafe { GLOBAL_EDGE_CALLER };
    edge_caller.acquire();
    let result = f(edge_caller);
    edge_caller.release();
    result
}
