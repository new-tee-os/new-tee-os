pub const PAGE_SIZE: usize = 0x1_000;
// also defined in keystone.lds
pub const KERNEL_BASE: usize = 0xFFFF_FFFF_C000_0000;
pub const KERNEL_PAGE_TABLE_PREALLOC: usize = 0x10_000;

pub const KERNEL_MIRROR_BASE: usize = 0xFFFF_FFFF_0000_0000;
pub const KERNEL_UTM_BASE: usize = 0xFFFF_FFFF_8000_0000;

pub const EPM_SIZE: usize = 0x100_000;
pub const UTM_SIZE: usize = 0x1_000;
pub const KERNEL_EPM_OFFSET: usize = 0x4_000;

// KERNEL_EPM_OFFSET + KERNEL_SIZE must be *smaller* than EPM_SIZE
