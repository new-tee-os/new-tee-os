use crate::{frame::TrapFrame, println, syscall::handle_syscall};

global_asm!(include_str!("asm/trap.S"));

#[no_mangle]
unsafe extern "C" fn trap_handler(frame: *mut TrapFrame) {
    use riscv::register::scause::{self, *};

    let trap_cause = scause::read().cause();
    match trap_cause {
        Trap::Exception(Exception::UserEnvCall) => {
            handle_syscall(frame);
        }
        _ => unknown_trap(),
    }
}

fn unknown_trap() -> ! {
    use riscv::register::*;

    println!("\n     ##### Kernel trapped! #####");
    println!("scause = {:?}", scause::read().cause());
    println!("sepc   = {:#X}", sepc::read());
    println!("stval  = {:#X}", stval::read());
    panic!("kernel trapped");
}
