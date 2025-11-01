use super::PrimalityTest;
use num_traits::{PrimInt, ToPrimitive, FromPrimitive};

/// Implementation of the Sieve of Eratosthenes primality test
///
/// This algorithm tests primality using trial division by all odd numbers
/// up to the square root of n. It's efficient for small to medium-sized numbers.
///
/// # Performance
///
/// - Time complexity: O(√n)
/// - Space complexity: O(1)
/// - Best for: Numbers < 10 million
#[derive(Default)]
pub struct SieveAlgorithm;

impl<N: PrimInt + ToPrimitive + FromPrimitive> PrimalityTest<N> for SieveAlgorithm {
    fn name(&self) -> &'static str {
        "Sieve of Eratosthenes"
    }

    fn is_prime(&self, n: N) -> bool {
        is_prime_sieve(n)
    }
}

/// Tests if a number is prime using trial division up to √n
///
/// This is a deterministic primality test that divides the number by all odd values
/// up to its square root. If none divide evenly, the number is prime.
///
/// # Arguments
///
/// * `n` - The number to test for primality
///
/// # Returns
///
/// `true` if n is prime, `false` if n is composite or less than 2
///
/// # Correctness
///
/// This function is 100% deterministic and always returns the correct result.
pub fn is_prime_sieve<N: PrimInt + ToPrimitive + FromPrimitive>(n: N) -> bool {
    let zero = N::zero();
    let one = N::one();
    let two = N::from_u64(2).unwrap();
    
    // Handle edge cases
    if n <= one {
        return false;
    }
    if n == two {
        return true;
    }
    if n % two == zero {
        return false;
    }

    // Check odd divisors up to sqrt(n)
    let n_f64 = n.to_f64().unwrap();
    let limit = N::from_u64(n_f64.sqrt() as u64 + 1).unwrap();
    
    let mut i = N::from_u64(3).unwrap();
    while i <= limit {
        if n % i == zero {
            return false;
        }
        i = i + two;
    }

    true
}
