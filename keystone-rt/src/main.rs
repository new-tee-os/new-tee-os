#![no_std]
#![no_main]
#![feature(asm, global_asm, panic_info_message)]

extern crate alloc;

use alloc::sync::Arc;
use hal::{
    arch::keystone::vm::{PageTableEntry, VirtAddr},
    edge::EdgeFile,
};
use kmalloc::{Kmalloc, LockedLinkedListHeap};
use log::debug;
use spin::Mutex;

mod elf;
mod entry;
mod frame;
mod klog;
mod panic;
mod syscall;
mod trap;
mod uart;
mod vm;

#[global_allocator]
static ALLOC: LockedLinkedListHeap = unsafe { LockedLinkedListHeap::uninit() };

static EPM_PHYS: spin::Once<usize> = spin::Once::new();

#[no_mangle]
extern "C" fn rt_main(vm_info: &vm::VmInfo) -> ! {
    // initialize EPM_PHYS
    EPM_PHYS.call_once(|| vm_info.epm_base);
    // initialize modules
    klog::klog_init().expect("failed to initialize klog module");
    unsafe {
        ALLOC.init(vm_info.free_virt as *mut u8, vm_info.free_size);
    }
    log::debug!("It did not crash!");

    // load U-mode program
    let entry;
    unsafe {
        // open ELF file
        let mut elf_file = elf::EdgeElfFile(EdgeFile::open("keystone-init"));

        // load & map ELF file
        let mem_mgr = vm::HeapPageManager::new();
        let mut root_page_table = vm::current_root_page_table();
        let elf =
            elf_loader::ElfFile::load(&mut elf_file, elf_loader::arch::RiscV, |from, size, to| {
                debug!(
                    "ELF loader: mapping ({:?} + {:#X}) -> {:#X}",
                    from, size, to
                );
                let from = from as usize;
                for i in 0..(size + 0xFFF) >> 12 {
                    root_page_table.map_4k(
                        VirtAddr(to + (i << 12)),
                        PageTableEntry::for_phys(mem_mgr.virt2phys(VirtAddr(from + (i << 12))))
                            .make_user()
                            .make_rwx(),
                    );
                }
            });
        entry = elf.entry() as usize;
    }

    let task = hal::task::Task::create(0x403000);
    let task_global = Arc::new(Mutex::new(task));
    let task_future = hal::task::TaskFuture::new(task_global.clone());

    // execute U-mode program
    unsafe {
        riscv::register::sepc::write(entry);
        riscv::register::sstatus::set_spp(riscv::register::sstatus::SPP::User);
    }
    executor::spawn(task_future);
    executor::run_until_idle();
    unreachable!()
}
