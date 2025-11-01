use erato::{ZetaAlgorithm, PrimalityTest};

fn main() {
    let zeta = ZetaAlgorithm;
    
    println!("13 is prime: {}", zeta.is_prime(13u64));
    println!("100 is prime: {}", zeta.is_prime(100u64));
    println!("1000000007 is prime: {}", zeta.is_prime(1_000_000_007u64));
    
    let largest_prime_u64 = 18_446_744_073_709_551_557u64;
    println!("18,446,744,073,709,551,557 is prime: {}", zeta.is_prime(largest_prime_u64));
}
