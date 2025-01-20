use super::elf_header::ElfHeader;
use super::instruction::{Instruction, Register};
use super::program_header::ProgramHeader;
use super::section_header::SectionHeader;
use crate::io::Buf;
use core::mem::offset_of;
use core::{panic, ptr};
use std::ffi::CStr;

const EXEC: u32 = 1;
const WRITE: u32 = 2;
const READ: u32 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Elf {
    header: ElfHeader,
    program_headers: [ProgramHeader; 3],
    section_headers: [SectionHeader; 4],
    code: [u8; 42],
    read_only_data: [u8; 12],
    strings_table: [u8; 25],
}

impl Elf {
    pub const SIZE: usize = size_of::<Self>();

    pub const PROGRAM_HEADERS_OFFSET: u64 = offset_of!(Self, program_headers) as u64;
    pub const SECTION_HEADERS_OFFSET: u64 = offset_of!(Self, section_headers) as u64;
    pub const STRINGS_TABLE_OFFSET: u64 = offset_of!(Self, strings_table) as u64;
    pub const READ_ONLY_DATA_OFFSET: u64 = offset_of!(Self, read_only_data) as u64;
    pub const CODE_OFFSET: u64 = offset_of!(Self, code) as u64;

    pub const STRINGS: [&CStr; 4] = [c".text", c".rodata", c".shshrtab", c""];

    pub const STRINGS_INDICIES: [u32; 4] = {
        let mut indices = [0; 4];
        let mut offset = 0;
        let mut index = 0;

        while index < Self::STRINGS.len() {
            let bytes = Self::STRINGS[index].to_bytes_with_nul();

            indices[index] = offset;
            offset += bytes.len() as u32;
            index += 1;
        }

        indices
    };

    pub const STRINGS_TABLE: [u8; 25] = {
        let mut table = [0; 25];
        let mut index = 0;
        let mut buf = Buf::new(&mut table);

        while index < Self::STRINGS.len() {
            let bytes = Self::STRINGS[index].to_bytes_with_nul();

            buf.write(bytes);
            index += 1;
        }

        table
    };

    pub const fn new() -> Self {
        let mut read_only_data = [0; 12];
        let mut code = [0; 42];

        {
            let input = *b"hello world~";
            let mut index = 0;

            while index < input.len() {
                read_only_data[index] = input[index];
                index += 1;
            }
        }

        let program_headers = [
            ProgramHeader {
                flags: READ,
                offset: 0,
                memory_size: 0x188,
                file_size: 0x188,
                virtual_addr: 0x0000000000400000,
                physical_addr: 0x0000000000400000,
                align: 0x1000,
                ..ProgramHeader::new()
            },
            ProgramHeader {
                flags: EXEC | READ,
                offset: Self::CODE_OFFSET,
                memory_size: code.len() as u64,
                file_size: code.len() as u64,
                virtual_addr: 0x0000000000401000,
                physical_addr: 0x0000000000401000,
                align: 0x1000,
                ..ProgramHeader::new()
            },
            ProgramHeader {
                flags: READ,
                offset: Self::READ_ONLY_DATA_OFFSET,
                memory_size: read_only_data.len() as u64,
                file_size: read_only_data.len() as u64,
                virtual_addr: 0x0000000000402000,
                physical_addr: 0x0000000000402000,
                align: 0x1000,
                ..ProgramHeader::new()
            },
        ];

        let section_headers = [
            SectionHeader {
                kind: super::section_header::Kind::Null,
                name: Self::STRINGS_INDICIES[3],
                ..SectionHeader::new()
            },
            SectionHeader {
                kind: super::section_header::Kind::ProgramData,
                offset: Self::CODE_OFFSET,
                size: code.len() as u64,
                name: Self::STRINGS_INDICIES[0],
                flags: 2 | 4,
                addr: program_headers[1].virtual_addr,
                addr_align: 1,
                ..SectionHeader::new()
            },
            SectionHeader {
                kind: super::section_header::Kind::ProgramData,
                offset: Self::READ_ONLY_DATA_OFFSET,
                size: read_only_data.len() as u64,
                name: Self::STRINGS_INDICIES[1],
                flags: 2,
                addr: program_headers[2].virtual_addr,
                addr_align: 1,
                ..SectionHeader::new()
            },
            SectionHeader {
                kind: super::section_header::Kind::StringTable,
                offset: Self::STRINGS_TABLE_OFFSET,
                size: Self::STRINGS_TABLE.len() as u64,
                name: Self::STRINGS_INDICIES[2],
                addr_align: 1,
                ..SectionHeader::new()
            },
        ];

        let instructions = [
            Instruction::mov(Register::Rax, 0x01),
            Instruction::mov(Register::Rdi, 0x01),
            Instruction::mov(Register::Rsi, section_headers[2].addr as u32),
            Instruction::mov(Register::Rdx, section_headers[2].size as u32),
            Instruction::syscall(),
            Instruction::mov(Register::Rax, 0x3C),
            Instruction::xor(Register::Rdi, Register::Rdi),
            Instruction::syscall(),
        ];

        let mut buf = Buf::new(&mut code);
        let mut instructions = instructions.as_slice();

        while let [instruction, instructions_rest @ ..] = instructions {
            let bytes = instruction.as_bytes();

            buf.write(bytes);
            instructions = instructions_rest;
        }

        Self {
            header: ElfHeader {
                entry_address: section_headers[1].addr,
                program_table: Self::PROGRAM_HEADERS_OFFSET,
                program_table_len: program_headers.len() as u16,
                section_table: Self::SECTION_HEADERS_OFFSET,
                section_table_len: section_headers.len() as u16,
                section_table_index: 3,
                ..ElfHeader::new()
            },
            program_headers,
            section_headers,
            strings_table: Self::STRINGS_TABLE,
            read_only_data,
            code,
        }
    }

    pub const fn as_bytes(&self) -> &[u8; Self::SIZE] {
        unsafe { &*ptr::from_ref(self).cast() }
    }
}
