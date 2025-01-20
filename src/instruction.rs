use crate::io::Buf;
use core::slice;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    Rax = 0xC0,
    Rdx = 0xC2,
    Rsi = 0xC6,
    Rdi = 0xC7,
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

        buf.write(b"\x48\xc7");
        buf.write(&[register as u8]);
        buf.write(&value.to_le_bytes());

        instruction.len = 2 + 1 + 4;
        instruction
    }

    pub const fn xor(left: Register, right: Register) -> Self {
        let mut instruction = Instruction::new();
        let mut buf = Buf::new(&mut instruction.bytes);

        match (left, right) {
            (Register::Rdi, Register::Rdi) => {
                buf.write(b"\x48\x31\xFF");
            }
            _ => unreachable!(),
        }

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
