use crate::println;

#[no_mangle]
extern "C" fn trap_handler() {
    println!("\n     ##### Kernel trapped! #####");
    println!("scause = {:?}", riscv::register::scause::read().cause());
    println!("sepc   = {:#X}", riscv::register::sepc::read());
    println!("stval  = {:#X}", riscv::register::stval::read());
    panic!("kernel trapped");
}
