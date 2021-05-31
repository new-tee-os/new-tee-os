extern crate libc;
use libc::{size_t,c_void};

pub use sgx_alloc::rsrvmem  as sgx_rsrvmem;


extern "C"{
    sgx_alloc_rsrv_mem_ex(
        desired_addr:*mut c_void, length : size_t
        )->*mut c_void;
}

impl RsrvMemAlloc{
    pub unsafe sgx_alloc_rsrv_mem_prm(desired_addr:u64,length:u64)->Result<*mut u64,Err>{
        let addr=sgx_alloc_rsrv_mem_ex(desired_addr as u64,length as u64);
        if addr == 0{
            Err
        }else{
            addr.try_into()
        }
    }
}
