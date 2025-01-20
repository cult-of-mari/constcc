use core::mem;

/// A buffer.
pub struct Buf<'a> {
    bytes: &'a mut [u8],
}

impl<'a> Buf<'a> {
    /// Create a new buffer from the provided byte slice.
    pub const fn new(bytes: &'a mut [u8]) -> Self {
        Self { bytes }
    }

    /// Remaining bytes in this buffer.
    pub const fn remaining(&self) -> usize {
        self.bytes.len()
    }

    /// Advance `amount` bytes.
    pub const fn advance(&mut self, amount: usize) {
        if self.remaining() < amount {
            advance_out_of_bounds();
        }

        advance(&mut self.bytes, amount);
    }

    /// Write `bytes` and advance.
    pub const fn write(&mut self, bytes: &[u8]) {
        copy_from_slice(self.bytes, bytes);

        self.advance(bytes.len());
    }
}

/// Advance `slice` by `amount`.
const fn advance(slice: &mut &mut [u8], amount: usize) {
    let (_, bytes) = mem::replace(slice, &mut []).split_at_mut(amount);

    *slice = bytes;
}

/// Copy `source` into `destination`.
const fn copy_from_slice(destination: &mut [u8], source: &[u8]) {
    let mut slices = (destination, source);

    while let ([destination, destination_rest @ ..], [source, source_rest @ ..]) = slices {
        *destination = *source;

        slices = (destination_rest, source_rest);
    }
}

#[track_caller]
const fn advance_out_of_bounds() -> ! {
    panic!("cannot advance out of bounds")
}
