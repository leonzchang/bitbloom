use alloc::vec::Vec;

/// A minimal growable bit vector backed by a byte array.
/// Provides constant-time access and mutation of individual bits.
#[derive(Debug, Clone)]
pub(crate) struct BitVec {
    bits: Vec<u8>,
}

const BITS_PER_BYTE: usize = 8;

impl BitVec {
    /// Creates a new `BitVec` with enough space for `bytes` bytes (i.e., `bytes * 8` bits).
    ///
    /// All bits are initialized to 0.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The number of bytes to allocate internally.
    #[inline]
    pub fn new(bytes: usize) -> Self {
        Self {
            bits: vec![0u8; bytes],
        }
    }

    /// Returns the total number of **bytes** in the internal storage.
    ///
    /// Note: To get total number of **bits**, multiply this by 8.
    #[inline]
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Checks if the bit at the given offset is set.
    ///
    /// # Arguments
    ///
    /// * `bit_offset` - The index of the bit to check (0-based).
    ///
    /// # Panics
    ///
    /// Panics if `bit_offset` is out of bounds.
    #[inline]
    pub fn contain(&self, bit_offset: usize) -> bool {
        let byte_offset = bit_offset / BITS_PER_BYTE;
        let bit_shift = bit_offset % BITS_PER_BYTE;

        debug_assert!(byte_offset < self.bits.len(), "bit_offset out of bounds");

        (self.bits()[byte_offset] & (1 << bit_shift)) != 0
    }

    /// Sets the bit at the given offset to `1`.
    ///
    /// # Arguments
    ///
    /// * `bit_offset` - The index of the bit to set (0-based).
    ///
    /// # Panics
    ///
    /// Panics if `bit_offset` is out of bounds.
    #[inline]
    pub fn set(&mut self, bit_offset: usize) {
        let byte_offset = bit_offset / BITS_PER_BYTE;
        let bit_shift = bit_offset % BITS_PER_BYTE;

        debug_assert!(byte_offset < self.bits.len(), "bit_offset out of bounds");

        self.bits_mut()[byte_offset] |= 1 << bit_shift;
    }

    /// Returns a read-only view of the internal byte array.
    #[inline]
    fn bits(&self) -> &[u8] {
        &self.bits
    }

    /// Returns a mutable view of the internal byte array.
    #[inline]
    fn bits_mut(&mut self) -> &mut [u8] {
        &mut self.bits
    }

    /// Returns the total number of bits in the bit vector (`len() * 8`).
    #[inline]
    pub fn capacity_in_bits(&self) -> usize {
        self.len() * BITS_PER_BYTE
    }

    /// Resets all bits to 0.
    #[inline]
    pub fn clear(&mut self) {
        for byte in &mut self.bits {
            *byte = 0;
        }
    }
}
