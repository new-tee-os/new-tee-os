use x86_64::registers::control::{Cr4, Cr4Flags};

pub fn enforce() {
    unsafe {
        Cr4::update(|flags| {
            flags.set(Cr4Flags::SUPERVISOR_MODE_ACCESS_PREVENTION, true);
            flags.set(Cr4Flags::SUPERVISOR_MODE_EXECUTION_PROTECTION, true);
            flags.set(Cr4Flags::USER_MODE_INSTRUCTION_PREVENTION, true);
        });
    }
}

fn set_smap(smap: bool) {
    unsafe {
        Cr4::update(|flags| {
            flags.set(Cr4Flags::SUPERVISOR_MODE_ACCESS_PREVENTION, smap);
        });
    }
}

pub fn with_smap_off<R>(f: impl FnOnce() -> R) -> R {
    set_smap(false);
    let result = f();
    set_smap(true);
    result
}
