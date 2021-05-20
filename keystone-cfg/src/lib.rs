#![no_std]

pub const PAGE_SIZE: usize = 0x1_000;
// also defined in keystone.lds
pub const KERNEL_BASE: usize = 0xFFFF_FFFF_C000_0000;
pub const KERNEL_PAGE_TABLE_PREALLOC: usize = 0x10_000;
// also defined in ks-user.lds
pub const USER_BASE: usize = 0x40_0000;

pub const EPM_SIZE: usize = 0x30_000;
pub const UTM_SIZE: usize = 0x1_000;
pub const KERNEL_EPM_OFFSET: usize = 0x10_000;

// KERNEL_EPM_OFFSET + KERNEL_SIZE must be *smaller* than EPM_SIZE
