# Bitbloom

<p align="center">
  <img src="https://raw.githubusercontent.com/leonzchang/bitbloom/refs/heads/main/assets/bitbloom.png" alt="bitbloom" width="50%">
</p>

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/leonzchang/bitbloom/blob/main/LICENSE)
[![crates.io](https://img.shields.io/crates/v/bitbloom)](
https://crates.io/crates/bitbloom)
[![docs.rs](https://img.shields.io/badge/docs-docs.rs-green)](https://docs.rs/bitbloom/latest/bitbloom/)

A `no_std` minimal Bloom filter for memory-constrained environment.

## Example

Add `bitbloom` to `Cargo.toml`:

```toml
[dependencies]
bitbloom = "0.1.0"
```

```rust
use bitbloom::Bloom;
use rand_pcg::Pcg64Mcg;
use rand_core::SeedableRng;

// Create a deterministic RNG
let mut rng = Pcg64Mcg::seed_from_u64(42);

// Create a Bloom filter for 1000 items with 1% false positive rate
let mut bloom = Bloom::new_with_rng(1000, 0.01, &mut rng);

// Insert items
bloom.set(&"hello");
bloom.set(&"world");

// Query membership
assert!(bloom.contain(&"hello"));
assert!(!bloom.contain(&"unknown"));
```

## References

- [Optimal number of bit and hash function](https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions)
- [Less Hashing, Same Performance:Building a Better Bloom Filter](https://www.eecs.harvard.edu/~michaelm/postscripts/rsa2008.pdf)
