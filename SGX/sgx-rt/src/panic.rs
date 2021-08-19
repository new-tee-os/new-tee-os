use core::panic::PanicInfo;
use crate::uart_println;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_println!("[:] {}",_info);
    loop {}
}
