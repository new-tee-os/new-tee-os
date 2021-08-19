extern crate libc;
use libc::{size_t,c_void};

extern "C"{
    fn sgx_alloc_rsrv_mem_ex(
        desired_addr:*mut c_void, length : size_t
        )->*mut c_void;
}
//TODO
pub unsafe fn sgx_alloc_rsrv_mem_prm(desired_addr:u64,length:u64)->Result<&'static mut [u64],i8>{
    let addr=sgx_alloc_rsrv_mem_ex(desired_addr as *mut c_void,length as usize);
    if addr == 0 as *mut c_void{
        Err(0)
    }else{
        //cast ptr first &[u8] has as_ptr() trait
        let addr:*mut u64=addr as *mut u64;
        Ok(core::slice::from_raw_parts_mut(addr,length as usize))
    }
}


use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static MYALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn default_handler(layout: core::alloc::Layout) -> ! {
    panic!("memory allocation of {} bytes failed", layout.size())
}
