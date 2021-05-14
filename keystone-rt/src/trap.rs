use crate::println;

global_asm!(include_str!("asm/trap.S"));

#[no_mangle]
extern "C" fn trap_handler() {
    unknown_trap();
}

fn unknown_trap() {
    println!("\n     ##### Kernel trapped! #####");
    println!("scause = {:?}", riscv::register::scause::read().cause());
    println!("sepc   = {:#X}", riscv::register::sepc::read());
    println!("stval  = {:#X}", riscv::register::stval::read());
    panic!("kernel trapped");
}
