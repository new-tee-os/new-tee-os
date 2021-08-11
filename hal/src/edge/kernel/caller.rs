use crate::edge::EdgeMemory;

/// The edge caller holder trait, which provides access to edge memory and
/// allows the user to issue edge calls.
///
/// In order to prevent concurrent access to edge callers, this object holds
/// a mutex provided by `GlobalEdgeCaller`. When the edge caller object gets
/// dropped, the mutex will be unlocked.
pub trait EdgeCallerHolder {
    fn edge_mem(&mut self) -> &mut EdgeMemory;
    unsafe fn edge_call(&mut self);
}

/// The global edge caller trait, which provides access to the actual `EdgeCaller`.
///
/// This object guarantees that edge calls are not concurrent by using a mutex.
///
/// The lifetime parameter `'c` indicates that all access to the edge memory
/// will only be granted within `'c`, because when the edge caller goes out of scope,
/// the mutex will be unlocked and all access to the edge caller will be revoked.
pub trait GlobalEdgeCaller<'c>: Sync {
    type Holder: EdgeCallerHolder + 'c;

    fn acquire(&'c self) -> Self::Holder;
}

use crate::sys::edge::GLOBAL_EDGE_CALLER;

/// Acquire the edge caller and do something with it (usually issuing edge calls).
pub fn with_edge_caller<F, V>(f: F) -> V
where
    F: FnOnce(&mut dyn EdgeCallerHolder) -> V,
{
    f(&mut GLOBAL_EDGE_CALLER.acquire())
}
