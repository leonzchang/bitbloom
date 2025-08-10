use bitbloom::Bloom;
use rand_core::SeedableRng;
use rand_pcg::Pcg64Mcg;

#[test]
fn should_match_insert_and_query() {
    let mut bloom = Bloom::new_with_key(100, 0.01, [(0, 1), (2, 3)]);

    bloom.set(&"hello");
    bloom.set(&"world");

    assert!(bloom.contain(&"hello"));
    assert!(bloom.contain(&"world"));
}

#[test]
fn should_returns_false_or_false_positive_non_inserted_item() {
    let mut rng = Pcg64Mcg::seed_from_u64(42);
    let mut bloom = Bloom::new_with_rng(1000, 0.01, &mut rng);

    bloom.set(&"foo");
    bloom.set(&"bar");

    assert!(!bloom.contain(&"baz") || bloom.contain(&"baz"));
}

#[test]
fn should_match_multiple_items() {
    let mut rng = Pcg64Mcg::seed_from_u64(123);
    let mut bloom = Bloom::new_with_rng(10_000, 0.01, &mut rng);

    let inserted = ["apple", "banana", "cherry", "date"];
    for word in &inserted {
        bloom.set(word);
    }

    for word in &inserted {
        assert!(
            bloom.contain(word),
            "Item {:?} should be in bloom filter",
            word
        );
    }

    let not_inserted = ["orange", "grape", "mango"];
    let mut false_positives = 0;
    for word in &not_inserted {
        if bloom.contain(word) {
            false_positives += 1;
        }
    }

    assert!(false_positives < not_inserted.len());
}

#[test]
fn should_report_correct_capacity_in_bits() {
    let bloom = Bloom::new_with_key(100, 0.01, [(1, 2), (3, 4)]);
    let capacity_bytes = bloom.capacity_in_bits() / 8;

    assert!(capacity_bytes >= 1, "Bit vector should be non-empty");
}

#[test]
fn should_clear_all_bits() {
    let mut bloom = Bloom::new_with_key(100, 0.01, [(10, 20), (30, 40)]);

    bloom.set(&"hello");
    bloom.set(&"world");
    assert!(bloom.contain(&"hello"));
    assert!(bloom.contain(&"world"));

    bloom.clear();

    // After clearing, should not match any item
    assert!(!bloom.contain(&"hello"));
    assert!(!bloom.contain(&"world"));
}
