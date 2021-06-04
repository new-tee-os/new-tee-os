use crate::edge::EdgeMemory;

pub trait EdgeCaller {
    fn edge_mem(&mut self) -> &mut EdgeMemory;
    unsafe fn edge_call(&self);
}

pub trait GlobalEdgeCaller<'c, C: EdgeCaller + 'c>: Sync {
    fn acquire(&'c self) -> C;
}

use crate::sys::GLOBAL_EDGE_CALLER;

pub fn with_edge_caller<F, V>(f: F) -> V
where
    F: FnOnce(&mut dyn EdgeCaller) -> V,
{
    f(&mut GLOBAL_EDGE_CALLER.acquire())
}
