use goblin::elf as elf;
use goblin::elf::{header::*,program_header::*};
use keystone_cfg::PAGE_SIZE;

#[feature("inline-asm")]
use riscv;

use std::fs;
use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
// static linked executable file (not PIE)
//  -static is usually enough, but some times it will output shared obj by default!?——what about riscv64
// mapped to phy addr with a linux pattern

//其他段的含义:note 放在load1里了
// GNUSTACK，GNURELOAD都是系统进程相关的了
//标准的类型转换，切片存储，vec数据迁移的方式

//检查其他可能的文件格式，映射page，no_std，分配堆栈，设置sp fp pc寄存器，填写libc空间入stack
//goblin 的feature怎么设定nostd之类的？
//程序怎么知道堆的？还是说由malloc在虚拟地址空间里自己分？用vmarea查询？

//没处理os释放内存，结束进程的部分,global_allocator结束之后会自己释放的吧
//try_into

#[inline]
fn get_pages(n:u64)->usize{
    let n:usize= n as usize;
    if n%PAGE_SIZE !=0 {
        1+n/PAGE_SIZE
    }else{
        n/PAGE_SIZE
    }
}

fn check_elf64_riscv(head: &Header)->Result<bool, bool>{
    //should assert_eq!
    if head.e_ident[0] != 0x7fu8{
        return Result::Err(false);
    }
    else if head.e_machine != EM_RISCV || head.e_type != ET_EXEC{
        return Result::Err(false);
    }
    return Result::Ok(true);
}

//return the stack pointer
fn prepare_libc_args()->usize{
    //give one page to the stack
    let mem:* mut [u8];
    unsafe {
        let _mem: * mut u8= alloc_zeroed(
            Layout::from_size_align(
                PAGE_SIZE, 
                32)//4*word/byte??????
                .unwrap()
        );
        
        mem =core::slice::from_raw_parts_mut(
            _mem,
            get_pages(PAGE_SIZE as _)<<12
        );
    }
    let stack_addr=0xBF800100;
    //create_mapping(mem as *mut [u8],stack_addr); //is the stack position determined by this?
    let argc:usize=0;
    let argv:[usize;0];
    //0
    let env:[usize;0];
    //0
    let aux:[usize;36];
    //0
    //strings
    let offset=(42/32 + 1 )*4*8;

    stack_addr+PAGE_SIZE-offset

}

pub fn load_elf64_riscv(path:&str){
    let data=fs::read(path).expect("file not exist");
    let bin=elf::Elf::parse(&data).expect("parse failed");
    
    println!("{:?}",bin.header);
    for seg in bin.program_headers.iter(){
        if seg.p_type == PT_LOAD{
            println!("{:?}",seg);
            // println!("{:?}",String::from(&bin.shdr_strtab[seg.sh_name]));
        }
    }

    let mut env:usize;
    let mut stv:usize;
    let mut enp:usize;
    let mut stp:usize;
    for seg in bin.program_headers.iter(){
        if seg.p_type == PT_LOAD{
            let mem: &mut [u8];
            unsafe {
                //allocate physical memory
                //set one page for each of the segment, for it requires much more support from OS
                //if we map one virt.
                let _mem: * mut u8= alloc(
                    Layout::from_size_align(
                        seg.p_memsz as usize, 
                        PAGE_SIZE)
                        .unwrap()
                );
                
                mem =core::slice::from_raw_parts_mut(
                    _mem,
                    get_pages(seg.p_memsz)
                );
            }
            stv=(seg.p_vaddr as usize)%PAGE_SIZE;
            env=stv+(seg.p_filesz) as usize;
            stp=(seg.p_offset as usize)%PAGE_SIZE;
            enp=(seg.p_offset+seg.p_filesz) as usize;
            mem[stv..env].copy_from_slice(&data[stp..enp]);
            
            //add these phy memories to the vm_area tree, and map page table
            //create_mapping(mem as *mut [u8],seg.p_vaddr);
        }
    }

    let sp=prepare_libc_args();
    let pc=bin.header.e_entry;
    //satp——页基寄存器
}