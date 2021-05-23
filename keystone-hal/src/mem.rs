use ::riscv::register::sstatus;

pub unsafe fn copy_from_user(kernel_mem: &mut [u8], user_mem: *const u8) {
    sstatus::set_sum();
    {
        let user_mem = core::slice::from_raw_parts(user_mem, kernel_mem.len());
        kernel_mem.copy_from_slice(user_mem);
    }
    sstatus::clear_sum();
}

pub unsafe fn copy_to_user(kernel_mem: &[u8], user_mem: *mut u8) {
    sstatus::set_sum();
    {
        let user_mem = core::slice::from_raw_parts_mut(user_mem, kernel_mem.len());
        user_mem.copy_from_slice(kernel_mem);
    }
    sstatus::clear_sum();
}
