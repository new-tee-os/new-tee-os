#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysAddr(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtAddr(pub usize);

impl PhysAddr {
    #[inline]
    pub fn ppn2(self) -> usize {
        (self.0 >> 30) & 0x3FFFFFF
    }

    #[inline]
    pub fn ppn1(self) -> usize {
        (self.0 >> 21) & 0x1FF
    }

    #[inline]
    pub fn ppn0(self) -> usize {
        (self.0 >> 12) & 0x1FF
    }

    #[inline]
    pub fn ppn(self) -> usize {
        (self.0 >> 12) & 0xFFFFFFFFFFFFFF
    }

    #[inline]
    pub fn page_offset(self) -> usize {
        self.0 & 0xFFF
    }

    #[inline]
    pub fn from_ppn_offset(ppn: usize, offset: usize) -> PhysAddr {
        assert!(ppn <= 0xFFFFFFFFFFF && offset <= 0xFFF);
        let mut addr = (ppn << 12) | offset;
        if (ppn & 0x80000000000) != 0 {
            addr |= 0xFF00000000000000;
        }
        PhysAddr(addr)
    }

    #[inline]
    pub fn from_parts(ppn2: usize, ppn1: usize, ppn0: usize, offset: usize) -> PhysAddr {
        assert!(ppn2 <= 0x3FFFFFF && ppn1 <= 0x1FF && ppn0 <= 0x1FF);
        let mut addr = (ppn2 << 30) | (ppn1 << 21) | (ppn0 << 12) | offset;
        if (ppn2 & 0x2000000) != 0 {
            addr |= 0xFF00000000000000;
        }
        PhysAddr(addr)
    }
}

impl VirtAddr {
    #[inline]
    pub fn vpn2(self) -> usize {
        (self.0 >> 30) & 0x1FF
    }

    #[inline]
    pub fn vpn1(self) -> usize {
        (self.0 >> 21) & 0x1FF
    }

    #[inline]
    pub fn vpn0(self) -> usize {
        (self.0 >> 12) & 0x1FF
    }

    #[inline]
    pub fn vpn(self) -> usize {
        self.0 >> 12
    }

    #[inline]
    pub fn page_offset(self) -> usize {
        self.0 & 0xFFF
    }

    #[inline]
    pub fn as_ptr<T>(self) -> *const T {
        self.0 as _
    }

    #[inline]
    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as _
    }

    #[inline]
    pub fn from_ptr<T>(ptr: *const T) -> VirtAddr {
        VirtAddr(ptr as usize)
    }
}
