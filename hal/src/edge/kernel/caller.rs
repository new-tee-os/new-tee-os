use crate::edge::EdgeMemory;

/// The edge caller trait, which provides access to edge memory and allows the
/// user to issue edge calls.
///
/// In order to prevent concurrent access to edge callers, this object holds
/// a mutex provided by `GlobalEdgeCaller`. When the edge caller object gets
/// dropped, the mutex will be unlocked.
pub trait EdgeCaller {
    fn edge_mem(&mut self) -> &mut EdgeMemory;
    unsafe fn edge_call(&self);
}

/// The global edge caller trait, which provides access to the actual `EdgeCaller`.
///
/// This object guarantees that edge calls are not concurrent by using a mutex.
pub trait GlobalEdgeCaller<'c, C: EdgeCaller + 'c>: Sync {
    fn acquire(&'c self) -> C;
}

use crate::sys::GLOBAL_EDGE_CALLER;

/// Acquire the edge caller and do something with it (usually issuing edge calls).
pub fn with_edge_caller<F, V>(f: F) -> V
where
    F: FnOnce(&mut dyn EdgeCaller) -> V,
{
    f(&mut GLOBAL_EDGE_CALLER.acquire())
}
