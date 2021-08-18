pub mod devices;
pub mod idt;
pub mod pic;

pub fn init() {
    idt::IDT.load();
    pic::init();
}
