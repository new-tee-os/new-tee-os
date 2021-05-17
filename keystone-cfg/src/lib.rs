#![no_std]

pub const PAGE_SIZE: usize = 0x1_000;
// also defined in keystone.lds
pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;
// also defined in ks-user.lds
pub const USER_BASE: usize = 0x40_0000;
// must be aligned to 4 kB boundary
pub const EPM_SIZE: usize = 0x30_000;
pub const UTM_SIZE: usize = 0x1_000;
pub const KERNEL_EPM_OFFSET: usize = 0x10_000;

// KERNEL_EPM_OFFSET + KERNEL_SIZE must be *smaller* than EPM_SIZE
