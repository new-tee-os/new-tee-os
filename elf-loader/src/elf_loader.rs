use goblin::elf::{header::*, program_header::*, Elf};

use alloc::alloc::{alloc, Layout};

const PAGE_SIZE: usize = 0x1000;

#[inline]
fn get_pages(n: u64) -> usize {
    let n: usize = n as usize;
    if n % PAGE_SIZE != 0 {
        1 + n / PAGE_SIZE
    } else {
        n / PAGE_SIZE
    }
}

fn check_elf64_riscv(head: &Header) {
    if &head.e_ident[0..4] != b"\x7FELF" {
        panic!("invalid ELF magic number: {:?}", &head.e_ident[0..4]);
    }
    if head.e_machine != EM_RISCV || head.e_type != ET_EXEC {
        panic!("unsupported architecture or ELF file type")
    }
    // check pass
}

pub trait MapperFn: FnMut(*const (), usize) {}
impl<T> MapperFn for T where T: FnMut(*const (), usize) {}

pub struct ElfFile {
    entry: u64,
}

impl ElfFile {
    pub fn load(data: &[u8], mut mapper: impl MapperFn) -> ElfFile {
        let elf = Elf::parse(data).expect("parse failed");
        check_elf64_riscv(&elf.header);

        for seg in elf.program_headers.iter() {
            if seg.p_type == PT_LOAD {
                let mem = unsafe {
                    let size = get_pages(seg.p_memsz) * PAGE_SIZE;
                    //allocate physical memory
                    //set one page for each of the segment, for it requires much more support from OS
                    //if we map one virt.
                    let mem_ptr: *mut u8 = alloc(Layout::from_size_align(size, PAGE_SIZE).unwrap());

                    core::slice::from_raw_parts_mut(mem_ptr, size)
                };

                // the virtual address where `mem` will be placed
                let load_addr = (seg.p_vaddr as usize) / PAGE_SIZE * PAGE_SIZE;
                let virt_off_begin = (seg.p_vaddr as usize) - load_addr;
                let virt_off_end = virt_off_begin + (seg.p_filesz as usize);
                let file_begin = seg.p_offset as usize;
                let file_end = file_begin + (seg.p_filesz as usize);
                mem[virt_off_begin..virt_off_end].copy_from_slice(&data[file_begin..file_end]);

                mapper(mem.as_ptr() as *const _, (seg.p_vaddr as usize) / PAGE_SIZE * PAGE_SIZE);
            }
        }

        ElfFile {
            entry: elf.header.e_entry,
        }
    }

    #[inline]
    pub fn entry(&self) -> u64 {
        self.entry
    }
}
