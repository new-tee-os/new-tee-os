extern crate sgx_types;
extern crate sgx_urts;
extern crate libc;

use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::fs::File;
use std::io::Read;
static ENCLAVE_FILE: &'static str = "enclave.signed.so";
const SHARED_MEM_SIZE:usize=0x400;

extern {
    fn rt_main(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
        sharemem: *mut u8, memsz: usize) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    let result = unsafe {
        let edge_mem = libc::malloc(SHARED_MEM_SIZE);

        // let mut kernel_file = File::open("sgx-rt.bin").expect("failed to open the bin");
        // let edge_mem_ref:&mut [u8]=core::slice::from_raw_parts_mut(edge_mem as *mut u8,SHARED_MEM_SIZE);
        // let bytes_read = kernel_file
        //     .read(edge_mem_ref)
        //     .expect("failed to read the bin");
        // if bytes_read == 0 {
        //     panic!("failed to read the bin");
        // }
        
        println!("[+] Shared memory allocated! Addr: 0X{:x}",edge_mem as usize);

        let mut retval = sgx_status_t::SGX_SUCCESS;
        rt_main(enclave.geteid(),
                    &mut retval,
                    edge_mem as _, 
                    SHARED_MEM_SIZE)
    };
    match result {
        sgx_status_t::SGX_SUCCESS => {
            println!("[+] SGX TEE OS operating successfully...");
        },
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
        }
    }
    enclave.destroy();
}
