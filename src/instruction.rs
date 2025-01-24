use crate::io::Buf;
use core::slice;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    Rax = 0, // Register code 0
    Rcx = 1, // Register code 1
    Rdx = 2, // Register code 2
    Rbx = 3, // Register code 3
    Rsp = 4, // Register code 4
    Rbp = 5, // Register code 5
    Rsi = 6, // Register code 6
    Rdi = 7, // Register code 7
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Instruction {
    bytes: [u8; 15],
    len: u8,
}

impl Instruction {
    pub const fn new() -> Self {
        Self {
            bytes: [0; 15],
            len: 0,
        }
    }

    pub const fn mov(register: Register, value: u32) -> Self {
        let mut instruction = Instruction::new();
        let mut buf = Buf::new(&mut instruction.bytes);

        buf.write(b"\x48\xC7"); // Opcode prefix + opcode
        let modrm = 0xC0 | (0 << 3) | (register as u8); // ModR/M: Mod=11 (register), Reg=0 (opcode extension /0 implied by C7), R/M=register
        buf.write(&[modrm]);
        buf.write(&value.to_le_bytes());

        instruction.len = 2 + 1 + 4;
        instruction
    }

    /// lea reg64, m (RIP-relative addressing)
    pub const fn lea_rip_relative(register: Register, displacement: u32) -> Self {
        let mut instruction = Instruction::new();
        let mut buf = Buf::new(&mut instruction.bytes);

        buf.write(b"\x48\x8D"); // Opcode prefix + opcode
        let modrm = 0x00 | ((register as u8) << 3) | 0x05; // ModR/M: Mod=00, Reg=register, R/M=101 (RIP-relative)
        buf.write(&[modrm]);
        buf.write(&displacement.to_le_bytes());

        instruction.len = 2 + 1 + 4;
        instruction
    }

    pub const fn xor(left: Register, right: Register) -> Self {
        let mut instruction = Instruction::new();
        let mut buf = Buf::new(&mut instruction.bytes);

        buf.write(b"\x48\x31"); // Opcode prefix + opcode
        let modrm = 0xC0 | ((left as u8) << 3) | (right as u8); // ModR/M: Mod=11 (register), Reg=left, R/M=right
        buf.write(&[modrm]);

        instruction.len = 3;
        instruction
    }

    pub const fn syscall() -> Self {
        let mut instruction = Instruction::new();
        let mut buf = Buf::new(&mut instruction.bytes);

        buf.write(b"\x0F\x05");

        instruction.len = 2;
        instruction
    }

    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.bytes.as_ptr(), self.len as usize) }
    }
}

// no const eq :explod:
const fn eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut pair = (left, right);

    while let ([left, left_rest @ ..], [right, right_rest @ ..]) = pair {
        if *left != *right {
            return false;
        }

        pair = (left_rest, right_rest);
    }

    true
}

// ok
const fn assert_encoding(instruction: Instruction, expected: &[u8]) {
    if !eq(instruction.as_bytes(), expected) {
        panic!("invalid encoding");
    }
}

const _: () = {
    assert_encoding(
        Instruction::mov(Register::Rax, 0x01),
        b"\x48\xC7\xC0\x01\x00\x00\x00",
    );

    assert_encoding(
        Instruction::mov(Register::Rax, 0x3C),
        b"\x48\xC7\xC0\x3c\x00\x00\x00",
    );

    assert_encoding(
        Instruction::mov(Register::Rdx, 0x12),
        b"\x48\xC7\xC2\x12\x00\x00\x00",
    );

    assert_encoding(
        Instruction::mov(Register::Rsi, 0x402000),
        b"\x48\xC7\xC6\x00\x20\x40\x00",
    );

    assert_encoding(
        Instruction::mov(Register::Rdi, 0x01),
        b"\x48\xC7\xC7\x01\x00\x00\x00",
    );

    assert_encoding(
        Instruction::xor(Register::Rdi, Register::Rdi),
        b"\x48\x31\xFF",
    );

    assert_encoding(Instruction::syscall(), b"\x0F\x05");
};
