use core::ptr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ProgramHeader {
    pub kind: u32,
    pub flags: u32,
    pub offset: u64,
    pub virtual_addr: u64,
    pub physical_addr: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub align: u64,
    //_padding: [u8; 8],
}

impl ProgramHeader {
    pub const SIZE: usize = size_of::<Self>();

    pub const fn new() -> Self {
        Self {
            kind: 1,    // PT_LOAD
            flags: 0x4, // exec | read (PF_X | PF_R)
            offset: 0,
            virtual_addr: 0,
            physical_addr: 0,
            file_size: 0,
            memory_size: 0,
            align: 0,
            //_padding: [0; 8],
        }
    }

    pub const fn as_bytes(&self) -> &[u8; Self::SIZE] {
        unsafe { &*ptr::from_ref(self).cast() }
    }
}

const _: () = {
    if ProgramHeader::SIZE != 56 {
        panic!("Invalid program header size");
    }
};
