use goblin::elf::{header::*, program_header::*, Elf};

use super::rsrvmalloc::sgx_rsrvmem;

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

fn check_elf64_x64(head: &Header) {
    if &head.e_ident[0..4] != b"\x7FELF" {
        panic!("invalid ELF magic number: {:?}", &head.e_ident[0..4]);
    }
    if head.e_machine != EM_X86_64 || head.e_type != ET_EXEC {
        panic!("unsupported architecture or EXEC file type for x86_64")
    }
    // check pass
}

pub struct ElfFile {
    entry: u64,
}

impl ElfFile {
    pub fn load(data: &[u8]) -> ElfFile {
        let elf = Elf::parse(data).expect("parse failed");
        check_elf64_x64(&elf.header);

        for seg in elf.program_headers.iter() {
            if seg.p_type == PT_LOAD {
                // the virtual address where `mem` will be placed
                let load_addr = (seg.p_vaddr as usize);
                let virt_off_begin = (seg.p_vaddr as usize) - load_addr;
                let virt_off_end = virt_off_begin + (seg.p_filesz as usize);
                let file_begin = seg.p_offset as usize;
                let file_end = file_begin + (seg.p_filesz as usize);

                let mem = unsafe {
                    //allocate physical memory
                    //set one page for each of the segment, for it requires much more support from OS
                    //if we map one virt.
                    if let Ok(mem_ptr) = sgx_alloc_rsrv_mem_prm(load_addr as u64, seg.p_memsz as u64){
                        core::slice::from_raw_parts_mut(mem_ptr, size)
                    }else{
                        panic!("fail to alloc rsrv memory to the dest address");
                    }
                };

                mem[virt_off_begin..virt_off_end].copy_from_slice(&data[file_begin..file_end]);
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

    //return the stack pointer
    //ignore the possibility that the stack overflow!
    fn prepare_libc_args() -> u64 {
        //give one page to the stack
        let mem = unsafe {
            let _mem: *mut u8 = alloc_zeroed(PAGE_SIZE);
            //core::slice::from_raw_parts_mut(_mem, get_pages(PAGE_SIZE as _) << 12)
        }
        let argc: usize = 0;
        let argv: [usize; 0];
        //0
        let env: [usize; 0];
        //0
        let aux: [usize; 36];
        //0
        //strings
        let offset :u64 = (42 / 32 + 1) * 4 * 8;

        unsafe{
            mem as u64 + PAGE_SIZE - offset
        }
    }

}