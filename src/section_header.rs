use core::ptr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Kind {
    Null = 0,
    ProgramData = 1,
    SymbolTable = 2,
    StringTable = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SectionHeader {
    pub name: u32,
    pub kind: Kind,
    pub flags: u32,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addr_align: u64,
    pub entry_size: u64,
}

impl SectionHeader {
    pub const SIZE: usize = size_of::<Self>();

    pub const fn new() -> Self {
        Self {
            name: 0,
            kind: Kind::ProgramData,
            flags: 0,
            addr: 0,
            offset: 0,
            size: 0,
            link: 0,
            info: 0,
            addr_align: 0,
            entry_size: 0,
        }
    }

    pub const fn as_bytes(&self) -> &[u8; Self::SIZE] {
        unsafe { &*ptr::from_ref(self).cast() }
    }
}

const _: () = {
    if SectionHeader::SIZE != 64 {
        panic!("invalid size");
    }
};
