use super::PrimalityTest;
use num_traits::{PrimInt, ToPrimitive, FromPrimitive, Unsigned};

/// Implementation of the Miller-Rabin primality test
///
/// This is a probabilistic primality test that becomes deterministic for 64-bit integers
/// by using a specific set of witness values. It's much faster than trial division
/// for large numbers.
///
/// # Performance
///
/// - Time complexity: O(k log³n) where k is the number of rounds
/// - Space complexity: O(1)
/// - Best for: Large numbers (> 1 billion)
///
/// # Correctness
///
/// For u64 integers, this implementation uses deterministic witnesses, making it
/// 100% accurate. False positives are impossible with these witnesses.
#[derive(Default)]
pub struct MillerRabinAlgorithm;

impl<N: PrimInt + ToPrimitive + FromPrimitive + Unsigned> PrimalityTest<N> for MillerRabinAlgorithm {
    fn name(&self) -> &'static str {
        "Miller-Rabin"
    }

    fn is_prime(&self, n: N) -> bool {
        is_prime_miller_rabin(n, 20)
    }
}

/// Miller-Rabin primality test with deterministic witnesses
///
/// This is a fast probabilistic primality test. For 64-bit integers,
/// a specific set of deterministic witnesses guarantees 100% accuracy.
///
/// The algorithm writes n-1 as 2^r × d (where d is odd) and then performs
/// witness tests using modular exponentiation.
///
/// # Arguments
///
/// * `n` - The number to test for primality
/// * `_k` - Number of rounds (ignored for u64 as we use deterministic witnesses)
///
/// # Returns
///
/// `true` if n is (definitely) prime, `false` if n is composite
///
/// # Correctness
///
/// This function is deterministic for all u64 integers and always returns
/// the mathematically correct result.
///
/// # References
///
/// See [Miller-Rabin Primality Test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
pub fn is_prime_miller_rabin<N: PrimInt + ToPrimitive + FromPrimitive + Unsigned>(n: N, _k: u32) -> bool {
    let zero = N::zero();
    let one = N::one();
    let two = N::from_u64(2).unwrap();
    let three = N::from_u64(3).unwrap();
    
    // Handle small cases
    if n <= one {
        return false;
    }
    if n == two || n == three {
        return true;
    }
    if n % two == zero {
        return false;
    }

    // Express n - 1 as 2^r * d where d is odd
    let mut d = n - one;
    let mut r = 0u32;
    while d % two == zero {
        d = d / two;
        r += 1;
    }

    // Deterministic set of witnesses for all u64 numbers
    let witnesses = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    // Test with each witness
    for &a in &witnesses {
        let a_n = N::from_u64(a).unwrap();
        if a_n >= n {
            continue;
        }
        if !check_composite(a_n, d, r, n) {
            return false;
        }
    }

    true
}

/// Checks if witness `a` proves that `n` is composite
///
/// Returns `true` if `n` passes the test with witness `a` (likely prime).
/// Returns `false` if `n` is definitely composite.
fn check_composite<N: PrimInt + ToPrimitive + FromPrimitive + Unsigned>(a: N, d: N, r: u32, n: N) -> bool {
    let one = N::one();
    let mut x = pow_mod(a, d, n);

    if x == one || x == n - one {
        return true;
    }

    for _ in 0..r - 1 {
        x = mul_mod(x, x, n);
        if x == n - one {
            return true;
        }
    }

    false
}

/// Computes (a × b) mod n using u128 to prevent overflow
fn mul_mod<N: PrimInt + ToPrimitive + FromPrimitive>(a: N, b: N, n: N) -> N {
    let a_u128 = a.to_u128().unwrap();
    let b_u128 = b.to_u128().unwrap();
    let n_u128 = n.to_u128().unwrap();
    
    let result = (a_u128 * b_u128) % n_u128;
    N::from_u128(result).unwrap()
}

/// Computes base^exp mod modulo using binary exponentiation
fn pow_mod<N: PrimInt + ToPrimitive + FromPrimitive + Unsigned>(mut base: N, mut exp: N, modulo: N) -> N {
    let zero = N::zero();
    let one = N::one();
    let two = N::from_u64(2).unwrap();
    
    let mut result = one;
    base = base % modulo;

    while exp > zero {
        if exp % two == one {
            result = mul_mod(result, base, modulo);
        }
        exp = exp >> 1;
        base = mul_mod(base, base, modulo);
    }

    result
}
