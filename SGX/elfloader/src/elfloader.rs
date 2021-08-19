/*  use goblin::{
        elf::{header, program_header, Elf, Header, ProgramHeader},
    };
    use super::rsrvmalloc::sgx_alloc_rsrv_mem_prm;
    // extern crate alloc;
    // use sgx_alloc::rsrvmem::alloc_zeroed;

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
        if head.e_machine != header::EM_X86_64 || head.e_type != header::ET_EXEC {
            panic!("unsupported architecture or EXEC file type for x86_64")
        }
    }

    pub struct ElfFile {
        entry: u64,
    }

    impl ElfFile {
        pub fn load(data: &[u8]) {// -> ElfFile
            let elf = goblin::elf::Elf::parse(data).expect("parse failed");

            // //cast two types of array through pointers
            // let data:&[u64] = unsafe{core::slice::from_raw_parts(data.as_ptr() as *const u64,data.len())};
            // // check_elf64_x64(&elf.header);


            // for seg in elf.program_headers.iter() {
            //     if seg.p_type == goblin::elf64::program_header::PT_LOAD {
            //         // the virtual address where `mem` will be placed
            //         let load_addr = seg.p_vaddr as usize;
            //         let virt_off_begin = (seg.p_vaddr as usize) - load_addr;
            //         let virt_off_end = virt_off_begin + (seg.p_filesz as usize);
            //         let file_begin = seg.p_offset as usize;
            //         let file_end = file_begin + (seg.p_filesz as usize);

            //         let mem = unsafe {
            //             //allocate physical memory
            //             //set one page for each of the segment, for it requires much more support from OS
            //             //if we map one virt.
            //             if let Ok(mem_ptr) = sgx_alloc_rsrv_mem_prm(load_addr as u64, seg.p_memsz as u64){
            //                 //core::slice::from_raw_parts_mut(mem_ptr,seg.p_memsz as usize)
            //                 mem_ptr
            //             }else{
            //                 panic!("fail to alloc rsrv memory to the dest address");
            //             }
            //         };

            //         mem[virt_off_begin..virt_off_end].copy_from_slice(&data[file_begin..file_end]);
            //     }
            // }

            // ElfFile {
            //     entry: elf.header.e_entry,
            // }
        }

        #[inline]
        pub fn entry(&self) -> u64 {
            self.entry
        }

        return the stack pointer
        ignore the possibility that the stack overflow!
        pub fn prepare_libc_args() -> u64 {
            // //give one page to the stack
            // let mem: *mut u8  = unsafe {
            //     alloc_zeroed(PAGE_SIZE as u32)
            //     //core::slice::from_raw_parts_mut(_mem, get_pages(PAGE_SIZE as _) << 12)
            // };
            // //wait for other libc preparations
            // // let argc: usize = 0;
            // // let argv: [usize; 0];
            // // //0
            // // let env: [usize; 0];
            // // //0
            // // let aux: [usize; 36];
            // //0
            // //strings
            // let offset :usize = (42 / 32 + 1) * 4 * 8;
            // //raw pointer addition
            // (mem as usize + PAGE_SIZE - offset) as u64
        }

    }
*/
use goblin::{
    container::{Container, Ctx, Endian},
    elf::{header, program_header, Elf, Header, ProgramHeader},
};

extern crate alloc;
use alloc::alloc::{GlobalAlloc,alloc,dealloc, Layout};

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
    if head.e_machine != header::EM_RISCV || head.e_type != header::ET_EXEC {
        panic!("unsupported architecture or ELF file type")
    }
    // check pass
}

pub trait MapperFn: FnMut(*const (), usize, usize) {}
impl<T> MapperFn for T where T: FnMut(*const (), usize, usize) {}

pub struct ElfFile {
    entry: u64,
}

pub trait ElfReader {
    fn read(&mut self, buf: &mut [u8]) -> usize;
    fn seek(&mut self, pos: u64);
}

impl ElfFile {
    pub fn load<R: ElfReader>(file: &mut R, mut mapper: impl MapperFn) -> ElfFile {
        // read ELF header
        let mut header = [0; core::mem::size_of::<Header>()];
        assert_eq!(file.read(&mut header), header.len());
        let header = Elf::parse_header(&header).expect("failed to parse ELF header");
        let mut elf = Elf::lazy_parse(header).expect("failed to parse ELF file");
        check_elf64_riscv(&elf.header);

        // create context
        let container = if header.e_ident[header::EI_CLASS] == header::ELFCLASS64 {
            Container::Big
        } else {
            Container::Little
        };
        let endian = Endian::from(header.e_ident[header::EI_DATA] == header::ELFDATA2LSB);
        let ctx = Ctx::new(container, endian);

        // read program header
        let mut program_headers =
            alloc::vec![0; (header.e_phnum as usize) * core::mem::size_of::<ProgramHeader>()];
        file.seek(header.e_phoff);
        assert_eq!(file.read(&mut program_headers), program_headers.len());
        elf.program_headers =
            ProgramHeader::parse(&program_headers, 0, header.e_phnum as usize, ctx)
                .expect("failed to parse program headers");

        for seg in elf.program_headers.iter() {
            if seg.p_type == program_header::PT_LOAD {
                // allocate memory using `alloc` API
                let mem = unsafe {
                    let size = get_pages(seg.p_memsz) * PAGE_SIZE;
                    let mem_ptr: *mut u8 = alloc(Layout::from_size_align(size, PAGE_SIZE).unwrap());

                    core::slice::from_raw_parts_mut(mem_ptr, size)
                };

                // compute the virtual address where `mem` will be placed
                let load_addr = (seg.p_vaddr as usize) / PAGE_SIZE * PAGE_SIZE;
                let virt_off_begin = (seg.p_vaddr as usize) - load_addr;
                let virt_off_end = virt_off_begin + (seg.p_filesz as usize); // MUST USE `p_filesz` as the size!
                let file_begin = seg.p_offset;
                // let file_end = file_begin + (seg.p_filesz as usize);

                // read data from the ELF file
                file.seek(file_begin);
                file.read(&mut mem[virt_off_begin..virt_off_end]);

                // map the memory block to the virtual address specified in the ELF file
                mapper(
                    mem.as_ptr() as *const _,
                    mem.len(),
                    (seg.p_vaddr as usize) / PAGE_SIZE * PAGE_SIZE,
                );
            }
        }

        ElfFile {
            entry: elf.header.e_entry,
        }
    }

    //return the stack pointer
    //ignore the possibility that the stack overflow!
    pub fn prepare_libc_args() -> u64 {
        return 1 as u64;

        // //give one page to the stack
        // let mem: *mut u8  = unsafe {
        //     alloc_zeroed(PAGE_SIZE as u32)
        //     //core::slice::from_raw_parts_mut(_mem, get_pages(PAGE_SIZE as _) << 12)
        // };
        // //wait for other libc preparations
        // // let argc: usize = 0;
        // // let argv: [usize; 0];
        // // //0
        // // let env: [usize; 0];
        // // //0
        // // let aux: [usize; 36];
        // //0
        // //strings
        // let offset :usize = (42 / 32 + 1) * 4 * 8;
        // //raw pointer addition
        // (mem as usize + PAGE_SIZE - offset) as u64
        
    }

    #[inline]
    pub fn entry(&self) -> u64 {
        self.entry
    }
}