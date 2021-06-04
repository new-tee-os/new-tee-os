const SBI_EXT_EXPERIMENTAL_KEYSTONE_ENCLAVE: usize = 0x08424b45;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_SM_STOP_ENCLAVE: usize = 3004;
const SBI_SM_EXIT_ENCLAVE: usize = 3006;
pub const STOP_TIMER_INTERRUPT: usize = 0;
pub const STOP_EDGE_CALL_HOST: usize = 1;

#[inline]
unsafe fn sbicall(ext: usize, which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let result;
    asm!(
        "ecall",

        in("a7") ext,
        in("a6") which,
        in("a0") arg0,
        in("a1") arg1,
        in("a2") arg2,
        lateout("a0") result,
    );
    result
}

pub fn putchar(ch: u8) {
    unsafe {
        sbicall(SBI_CONSOLE_PUTCHAR, 0, ch as usize, 0, 0);
    }
}

pub fn stop_enclave(req: usize) -> usize {
    unsafe {
        sbicall(
            SBI_EXT_EXPERIMENTAL_KEYSTONE_ENCLAVE,
            SBI_SM_STOP_ENCLAVE,
            req,
            0,
            0,
        )
    }
}

pub fn exit_enclave(retval: usize) {
    unsafe {
        sbicall(
            SBI_EXT_EXPERIMENTAL_KEYSTONE_ENCLAVE,
            SBI_SM_EXIT_ENCLAVE,
            retval,
            0,
            0,
        );
    }
}
