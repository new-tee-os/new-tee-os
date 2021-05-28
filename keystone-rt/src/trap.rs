use log::debug;

use crate::{frame::TrapFrame, syscall::handle_syscall, uart_println};

global_asm!(include_str!("asm/trap.S"));

#[no_mangle]
unsafe extern "C" fn trap_handler(frame: *mut TrapFrame) {
    use riscv::register::scause::{self, *};

    let trap_cause = scause::read().cause();
    match trap_cause {
        Trap::Exception(Exception::UserEnvCall) => {
            handle_syscall(frame);
        }
        Trap::Exception(Exception::LoadPageFault | Exception::StorePageFault) => {
            let addr = riscv::register::stval::read();
            debug!("Page fault at address {:#X}", addr);
            crate::vm::handle_page_fault_at(addr);
            // now just redo the errorneous instruction
        }
        _ => unknown_trap(),
    }
}

fn unknown_trap() -> ! {
    use riscv::register::*;

    uart_println!("\n     ##### Kernel trapped! #####");
    uart_println!("scause = {:?}", scause::read().cause());
    uart_println!("sepc   = {:#X}", sepc::read());
    uart_println!("stval  = {:#X}", stval::read());
    panic!("kernel trapped");
}
