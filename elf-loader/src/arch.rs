use goblin::elf::header;

pub trait ElfArch {
    const E_MACHINE: u16;
}

pub struct RiscV;
pub struct X86_64;

impl ElfArch for RiscV {
    const E_MACHINE: u16 = header::EM_RISCV;
}

impl ElfArch for X86_64 {
    const E_MACHINE: u16 = header::EM_X86_64;
}
