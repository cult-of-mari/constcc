use super::{program_header::ProgramHeader, section_header::SectionHeader};
use core::ptr;

macro_rules! impl_enum {
    {
        $vis:vis enum $ident:ident : $repr:ident {
            $($variant:ident = $value:literal,)*
        }
    } => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr($repr)]
        $vis enum $ident {
            $($variant = $value, )*
        }
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum BitWidth {
    ThirtyTwo = 1,
    SixtyFour = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Endian {
    Little = 1,
    Big = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Version {
    One = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Env {
    SysV = 0,
}

impl_enum! {
    pub enum EnvVersion : u8 {
        Zero = 0,
    }
}

impl_enum! {
    pub enum Kind : u16 {
        Executable = 2,
    }
}

impl_enum! {
    pub enum Arch : u16 {
        X86_64 = 0x3E,
    }
}

impl_enum! {
    pub enum Version2 : u32 {
        One = 1,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ElfHeader {
    pub magic: [u8; 4],
    pub bit_width: BitWidth,
    pub endian: Endian,
    pub version: Version,
    pub env: Env,
    pub env_version: EnvVersion,
    pub _padding: [u8; 7],
    pub kind: Kind,
    pub arch: Arch,
    pub version2: Version2,
    pub entry_address: u64,
    pub program_table: u64,
    pub section_table: u64,
    pub flags: u32,
    pub size: u16,
    pub program_table_size: u16,
    pub program_table_len: u16,
    pub section_table_size: u16,
    pub section_table_len: u16,
    pub section_table_index: u16,
    pub _padding2: [u8; 0],
}

impl ElfHeader {
    pub const SIZE: usize = size_of::<Self>();

    pub const fn new() -> Self {
        Self {
            magic: *b"\x7FELF",
            bit_width: BitWidth::SixtyFour,
            endian: Endian::Little,
            version: Version::One,
            env: Env::SysV,
            env_version: EnvVersion::Zero,
            _padding: [0; 7],
            kind: Kind::Executable,
            arch: Arch::X86_64,
            version2: Version2::One,
            entry_address: 0,
            program_table: 0,
            section_table: 0,
            flags: 0,
            size: Self::SIZE as u16,
            program_table_size: ProgramHeader::SIZE as u16,
            program_table_len: 0,
            section_table_size: SectionHeader::SIZE as u16,
            section_table_len: 0,
            section_table_index: 0,
            _padding2: [0; 0],
        }
    }

    pub const fn as_bytes(&self) -> &[u8; Self::SIZE] {
        unsafe { &*ptr::from_ref(self).cast::<[u8; Self::SIZE]>() }
    }
}

const _: () = {
    let elf = ElfHeader::new();

    if elf.program_table_size != 56 {
        panic!("invalid prog tbl sz");
    }

    if elf.section_table_size != 64 {
        panic!("invalid sec tbl sz");
    }

    if ElfHeader::SIZE != 64 {
        panic!("Invalid ELF header size");
    }
};

/*impl Write<ElfHeader> {
    pub const fn write(value: &ElfHeader, buf: &mut Buf<'_>) {
        buf.write(&value.magic);
        buf.write(&value.bit_width);
        buf.write(&value.endian);
        buf.write(&value.version);
        buf.write(&value.env);
        buf.write(&value.env_version);
        buf.write(&value._padding);
        buf.write(&value.kind);
        buf.write(&value.arch);
        buf.write(&value.version2);
    }
}*/
