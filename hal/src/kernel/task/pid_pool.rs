use alloc::collections::BTreeSet;

use super::Pid;

pub struct PidPool {
    pool: BTreeSet<Pid>,
    next_free: Pid,
}

impl PidPool {
    pub const fn new() -> PidPool {
        PidPool {
            pool: BTreeSet::new(),
            next_free: 1,
        }
    }

    pub fn alloc(&mut self) -> Pid {
        // if the pool is not empty, pop the first PID in the pool
        if let Some(pid) = self.pool.pop_first() {
            pid
        } else {
            // or else, allocate the next free PID
            let result = self.next_free;
            self.next_free += 1;
            result
        }
    }

    pub fn free(&mut self, pid: Pid) {
        assert!(pid < self.next_free);
        // return the PID to the pool
        assert!(self.pool.insert(pid));
    }
}
