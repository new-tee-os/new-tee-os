use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, RawFd};

mod ioctl;

pub use ioctl::RuntimeParams;

pub struct KeystoneDev {
    _file: File,
    fd: RawFd,
    eid: Option<usize>,
    phys_addr: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum EnclaveStatus {
    EdgeCallHost,
    Interrupted,
    Done(usize),
    UnknownError(usize),
}

const KEYSTONE_DEV_PATH: &str = "/dev/keystone_enclave";

#[allow(unused)]
impl KeystoneDev {
    pub fn open() -> std::io::Result<KeystoneDev> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(KEYSTONE_DEV_PATH)?;
        let fd = file.as_raw_fd();
        Ok(KeystoneDev {
            _file: file,
            fd,
            eid: None,
            phys_addr: 0,
        })
    }

    unsafe fn keystone_ioctl<T>(
        &self,
        ioctl: unsafe fn(RawFd, T) -> nix::Result<nix::libc::c_int>,
        params: T,
    ) -> nix::Result<()> {
        ioctl(self.fd, params).and_then(|err| {
            if err == 0 {
                Ok(())
            } else {
                dbg!(err);
                Err(nix::Error::from_errno(nix::errno::Errno::EINVAL))
            }
        })
    }

    pub fn create(&mut self, min_pages: usize) -> nix::Result<()> {
        // expect_none is unstable
        if let Some(_) = self.eid {
            panic!("enclave already created");
        }
        let mut ioctl_req = ioctl::CreateEnclave {
            min_pages,
            ..Default::default()
        };
        unsafe {
            self.keystone_ioctl(ioctl::create_enclave, &mut ioctl_req)?;
        }
        self.eid = Some(ioctl_req.eid);
        self.phys_addr = ioctl_req.pt_ptr;
        Ok(())
    }

    pub fn phys_addr(&self) -> usize {
        self.phys_addr
    }

    pub fn init_utm(&self, size: usize) -> nix::Result<usize> {
        let mut ioctl_req = ioctl::CreateEnclave {
            eid: self.eid.expect("enclave not yet created"),
            params: ioctl::RuntimeParams {
                untrusted_size: size,
                ..Default::default()
            },
            ..Default::default()
        };
        unsafe {
            self.keystone_ioctl(ioctl::utm_init, &mut ioctl_req)?;
        }
        Ok(ioctl_req.utm_free_ptr)
    }

    pub fn finalize(
        &self,
        rt_phys_addr: usize,
        eapp_phys_addr: usize,
        free_phys_addr: usize,
        params: ioctl::RuntimeParams,
    ) -> nix::Result<()> {
        let mut ioctl_req = ioctl::CreateEnclave {
            eid: self.eid.expect("enclave not yet created"),
            runtime_paddr: rt_phys_addr,
            user_paddr: eapp_phys_addr,
            free_paddr: free_phys_addr,
            params,
            ..Default::default()
        };
        unsafe { self.keystone_ioctl(ioctl::finalize_enclave, &mut ioctl_req) }
    }

    pub unsafe fn map_mem(&self, addr: usize, size: usize) -> nix::Result<*mut ()> {
        use nix::sys::mman::*;
        mmap(
            std::ptr::null_mut(),
            size,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            self.fd,
            addr as i64,
        )
        .map(|ptr| ptr as *mut ())
    }

    pub unsafe fn unmap_mem(&self, addr: *mut (), size: usize) -> nix::Result<()> {
        use nix::sys::mman::*;
        munmap(addr as _, size)
    }

    fn map_enclave_status(ioctl_req: &ioctl::RunEnclave) -> EnclaveStatus {
        use ioctl::*;
        use EnclaveStatus::*;
        match ioctl_req.error {
            KEYSTONE_ENCLAVE_DONE => Done(ioctl_req.value),
            KEYSTONE_ENCLAVE_EDGE_CALL_HOST => EdgeCallHost,
            KEYSTONE_ENCLAVE_INTERRUPTED => Interrupted,
            _ => UnknownError(ioctl_req.error),
        }
    }

    pub fn run(&self) -> nix::Result<EnclaveStatus> {
        let mut ioctl_req = ioctl::RunEnclave {
            eid: self.eid.expect("enclave not yet created"),
            ..Default::default()
        };
        unsafe {
            ioctl::run_enclave(self.fd, &mut ioctl_req)?;
        }
        Ok(Self::map_enclave_status(&ioctl_req))
    }

    pub fn resume(&self) -> nix::Result<EnclaveStatus> {
        let mut ioctl_req = ioctl::RunEnclave {
            eid: self.eid.expect("enclave not yet created"),
            ..Default::default()
        };
        unsafe {
            ioctl::resume_enclave(self.fd, &mut ioctl_req)?;
        }
        Ok(Self::map_enclave_status(&ioctl_req))
    }

    pub fn destroy(&mut self) -> nix::Result<()> {
        let mut ioctl_req = ioctl::CreateEnclave {
            eid: self.eid.expect("enclave not yet created"),
            ..Default::default()
        };
        unsafe { self.keystone_ioctl(ioctl::destroy_enclave, &ioctl_req) }
    }
}
