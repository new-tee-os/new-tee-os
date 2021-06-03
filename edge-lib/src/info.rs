#[repr(C)]
#[derive(Clone, Copy)]
pub enum EdgeCallInfo {
    SyscallWrite { fd: u64, len: u64 },
    FileOpen,
    FileRead { file_obj: u64 },
    FileGetSize { file_obj: u64 },
    FileClose { file_obj: u64 },
    U64Info(u64),
}

const STRUCT_LEN: usize = core::mem::size_of::<EdgeCallInfo>();

static_assertions::const_assert!(STRUCT_LEN <= crate::EDGE_CALL_INFO_SIZE);

// note: maybe unsafe, since these functions operate on the
// underlying binary representation of the object
impl EdgeCallInfo {
    pub fn write_to(&self, dest: &mut [u8]) {
        let self_as_data =
            unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, STRUCT_LEN) };
        dest[0..STRUCT_LEN].copy_from_slice(self_as_data);
    }

    pub unsafe fn read_from(src: &[u8]) -> EdgeCallInfo {
        let mut result = core::mem::MaybeUninit::<EdgeCallInfo>::uninit();
        let result_as_data =
            core::slice::from_raw_parts_mut(result.as_mut_ptr() as *mut u8, STRUCT_LEN);
        result_as_data.copy_from_slice(&src[0..STRUCT_LEN]);
        result.assume_init()
    }
}

impl From<u64> for EdgeCallInfo {
    fn from(value: u64) -> Self {
        EdgeCallInfo::U64Info(value)
    }
}

impl From<EdgeCallInfo> for u64 {
    fn from(value: EdgeCallInfo) -> Self {
        if let EdgeCallInfo::U64Info(value) = value {
            value
        } else {
            panic!("the info is not a u64 variant")
        }
    }
}
