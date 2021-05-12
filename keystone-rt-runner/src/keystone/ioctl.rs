use nix::{ioctl_read, ioctl_write_ptr};

// keystone-sdk/include/host/keystone_user.h
// keystone-sdk/src/host/KeystoneDevice.cpp

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct RuntimeParams {
    pub runtime_entry: usize,
    pub user_entry: usize,
    pub untrusted_ptr: usize,
    pub untrusted_size: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct CreateEnclave {
    pub eid: usize,
    pub min_pages: usize,
    pub runtime_vaddr: usize,
    pub user_vaddr: usize,
    pub pt_ptr: usize,
    pub utm_free_ptr: usize,
    pub epm_paddr: usize,
    pub utm_paddr: usize,
    pub runtime_paddr: usize,
    pub user_paddr: usize,
    pub free_paddr: usize,
    pub epm_size: usize,
    pub utm_size: usize,
    pub params: RuntimeParams,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct RunEnclave {
    pub eid: usize,
    pub error: usize,
    pub value: usize,
}

const KEYSTONE_IOC_MAGIC: u8 = 0xa4;

ioctl_read!(create_enclave, KEYSTONE_IOC_MAGIC, 0x00, CreateEnclave);
ioctl_write_ptr!(destroy_enclave, KEYSTONE_IOC_MAGIC, 0x01, CreateEnclave);
ioctl_read!(run_enclave, KEYSTONE_IOC_MAGIC, 0x04, RunEnclave);
ioctl_read!(resume_enclave, KEYSTONE_IOC_MAGIC, 0x05, RunEnclave);
ioctl_read!(finalize_enclave, KEYSTONE_IOC_MAGIC, 0x06, CreateEnclave);
ioctl_read!(utm_init, KEYSTONE_IOC_MAGIC, 0x07, CreateEnclave);

pub const KEYSTONE_ENCLAVE_DONE: usize = 0;
pub const KEYSTONE_ENCLAVE_INTERRUPTED: usize = 100002;
pub const KEYSTONE_ENCLAVE_EDGE_CALL_HOST: usize = 100011;
