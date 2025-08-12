use core::{
    f64::consts::LN_2,
    hash::{Hash, Hasher},
};

use libm::{ceil, log, log2, pow};
use rand_core::RngCore;
use siphasher::sip::SipHasher13;

use crate::bit_vec::BitVec;

/// A probabilistic Bloom filter for membership testing with configurable
/// false positive rate and no false negatives.
///
/// This implementation uses double hashing based on two SipHash-1-3 hashers
/// seeded with independent keys for reproducibility and security.
///
/// The bit vector size and number of hash functions are calculated based on
/// the expected number of items and desired false positive rate.
#[derive(Debug, Clone)]
pub struct Bloom {
    bits: BitVec,
    hash_fn_number: usize,
    hashers: [SipHasher13; 2],
}

impl Bloom {
    /// Creates a new Bloom filter with the specified expected number of items,
    /// desired false positive rate, and explicit SipHash keys.
    ///
    /// # Arguments
    ///
    /// * `items` - Expected number of items to be inserted (must be > 0).
    /// * `err_rate` - Desired false positive probability (0 < err_rate < 1).
    /// * `keys` - Array of two `(u64, u64)` tuples used as keys for SipHash.
    ///
    /// # Panics
    ///
    /// Panics if `items` is zero or if `err_rate` is not in (0,1).
    pub fn new_with_key(items: usize, err_rate: f64, keys: [(u64, u64); 2]) -> Self {
        let bits_size = Self::calculate_bits_vec_size(items, err_rate);
        let hash_fn_number = Self::calculate_hash_fn_number(err_rate);
        let [key0, key1] = keys;

        let hashers = [
            SipHasher13::new_with_keys(key0.0, key0.1),
            SipHasher13::new_with_keys(key1.0, key1.1),
        ];

        Self {
            bits: BitVec::new(bits_size),
            hash_fn_number,
            hashers,
        }
    }

    /// Creates a new Bloom filter with the specified expected number of items,
    /// false positive rate, and a random number generator to seed SipHash keys.
    ///
    /// # Arguments
    ///
    /// * `items` - Expected number of items to be inserted (must be > 0).
    /// * `err_rate` - Desired false positive probability (0 < err_rate < 1).
    /// * `rng` - Mutable reference to a random number generator implementing `RngCore`.
    ///
    /// # Panics
    ///
    /// Panics if `items` is zero or if `err_rate` is not in (0,1).
    pub fn new_with_rng<R: RngCore>(items: usize, err_rate: f64, rng: &mut R) -> Self {
        let hash_fn_number = Self::calculate_hash_fn_number(err_rate);
        let keys = [
            (rng.next_u64(), rng.next_u64()),
            (rng.next_u64(), rng.next_u64()),
        ];

        let hashers = [
            SipHasher13::new_with_keys(keys[0].0, keys[0].1),
            SipHasher13::new_with_keys(keys[1].0, keys[1].1),
        ];

        Self {
            bits: BitVec::new(Self::calculate_bits_vec_size(items, err_rate)),
            hash_fn_number,
            hashers,
        }
    }

    /// Inserts an item into the Bloom filter.
    ///
    /// # Arguments
    ///
    /// * `item` - A reference to the item to insert, which must implement `Hash`.
    pub fn set<T>(&mut self, item: &T)
    where
        T: Hash,
    {
        let (h1, h2) = self.bloom_hash(item);
        for i in 0..self.hash_fn_number {
            let index = self.get_index((h1, h2), i as u64);
            self.bits.set(index);
        }
    }

    /// Checks if an item is possibly in the Bloom filter.
    ///
    /// Returns `true` if the item may be in the set, or `false` if definitely not.
    ///
    /// # Arguments
    ///
    /// * `item` - A reference to the item to check, which must implement `Hash`.
    pub fn contain<T>(&self, item: &T) -> bool
    where
        T: Hash,
    {
        let (h1, h2) = self.bloom_hash(item);
        for i in 0..self.hash_fn_number {
            let index = self.get_index((h1, h2), i as u64);
            if !self.bits.contain(index) {
                return false;
            }
        }
        true
    }

    /// Hashes an item into two base hash values using the internal SipHash instances.
    ///
    /// This is used to implement double hashing for generating multiple hash values.
    #[inline]
    fn bloom_hash<T>(&self, item: &T) -> (u64, u64)
    where
        T: Hash,
    {
        let mut hasher1 = self.hashers[0];
        let mut hasher2 = self.hashers[1];

        item.hash(&mut hasher1);
        item.hash(&mut hasher2);

        (hasher1.finish(), hasher2.finish())
    }

    /// Computes the bit index for the `i`th hash function using double hashing:
    ///
    /// `g_i(x) = (h1(x) + i * h2(x)) mod m` where `m` is the bit vector size.
    #[inline]
    fn get_index(&self, (h1, h2): (u64, u64), i: u64) -> usize {
        let len = self.bits.len() as u64;
        (h1.wrapping_add(i.wrapping_mul(h2)) % len) as usize
    }

    /// Calculates the minimum size of the bit vector (in bytes) needed to achieve
    /// the specified false positive rate given the expected number of items.
    ///
    /// Formula used:
    /// ```text
    /// m = - (n * ln ε) / (8 * (ln 2)^2)
    /// ```
    ///
    /// where `n` is number of items, `ε` is false positive rate, and `m` is bit vector size in bytes.
    ///
    /// # Panics
    ///
    /// Panics if `items == 0` or `fp_rate` not in `(0,1)`.
    #[inline]
    fn calculate_bits_vec_size(items: usize, fp_rate: f64) -> usize {
        assert!(items > 0, "Number of items must be > 0");
        assert!(
            (0.0..1.0).contains(&fp_rate),
            "False positive rate must be between 0 and 1"
        );

        ceil(-((items as f64 * log(fp_rate)) / (pow(LN_2, 2.0) * 8.0))) as usize
    }

    /// Calculates the optimal number of hash functions needed for the given false positive rate.
    ///
    /// Formula:
    /// ```text
    /// k = ceil(-log_2(ε))
    /// ```
    ///
    /// where `ε` is false positive rate, and `k` is number of hash functions.
    #[inline]
    fn calculate_hash_fn_number(fp_rate: f64) -> usize {
        ceil(-log2(fp_rate)) as usize
    }

    /// Returns the total capacity of the Bloom filter in **bits**.
    ///
    /// This is equal to the number of bytes in the underlying bit vector multiplied by 8.
    #[inline]
    pub fn capacity_in_bits(&self) -> usize {
        self.bits.capacity_in_bits()
    }

    /// Clears all bits in the Bloom filter, effectively resetting it.
    ///
    /// After calling this, the filter will behave as if it's empty.
    #[inline]
    pub fn clear(&mut self) {
        self.bits.clear();
    }
}
