use wasm_bindgen::prelude::*;

/// Erato - A library for primality testing algorithms
///
/// This library provides multiple implementations of primality testing algorithms
/// with a unified trait-based interface for easy comparison and benchmarking.
///
/// # Features
///
/// - **Multiple algorithms**: Sieve of Eratosthenes and Miller-Rabin
/// - **Unified interface**: Implement `PrimalityTest` trait for custom algorithms
/// - **Registry system**: Easily manage and compare all algorithms
/// - **Well-tested**: Comprehensive test coverage

pub mod algorithms;

pub use algorithms::sieve::{is_prime_sieve, SieveAlgorithm};
pub use algorithms::miller_rabin::{is_prime_miller_rabin, MillerRabinAlgorithm};
pub use algorithms::zeta::{is_prime_zeta, ZetaAlgorithm};
pub use algorithms::{PrimalityTest, PrimalityRegistry};

#[wasm_bindgen]
pub fn is_prime(n: u64) -> bool {
    is_prime_zeta(n)
}

#[wasm_bindgen]
pub fn prime_count_data(max_n: u64) -> Vec<u64> {
    let mut data = Vec::new();
    let mut count = 0u64;
    
    for n in 2..=max_n {
        if is_prime_zeta(n) {
            count += 1;
        }
        let step = if max_n > 10000 { 100 } else if max_n > 1000 { 10 } else { 1 };
        if n % step == 0 || n == max_n {
            data.push(n);
            data.push(count);
        }
    }
    
    data
}
