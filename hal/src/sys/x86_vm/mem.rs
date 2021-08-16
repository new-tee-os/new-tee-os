pub unsafe fn copy_from_user(kernel_mem: &mut [u8], user_mem: *const u8) {
    // TODO: implement SMAP
    let user_mem = core::slice::from_raw_parts(user_mem, kernel_mem.len());
    kernel_mem.copy_from_slice(user_mem);
}

pub unsafe fn copy_to_user(kernel_mem: &[u8], user_mem: *mut u8) {
    // TODO: implement SMAP
    let user_mem = core::slice::from_raw_parts_mut(user_mem, kernel_mem.len());
    user_mem.copy_from_slice(kernel_mem);
}
