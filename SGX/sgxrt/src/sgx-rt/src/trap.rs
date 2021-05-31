#![no_std]

use sgx_trts::veh::*;
use sgx_types::sgx_cpu_context_t;

use super::syscall;

pub fn trap_handler_init() {
    match rsgx_register_exception_handler(0, syscall::handle_syscall) {
        Some() => (), 
        _ => debug!("fail to register syscall {}", i),
    }
}
