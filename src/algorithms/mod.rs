use num_traits::{PrimInt, ToPrimitive, FromPrimitive, Unsigned};

/// Sieve of Eratosthenes primality test
pub mod sieve;

/// Miller-Rabin primality test
pub mod miller_rabin;

/// Riemann Hypothesis-based primality test
pub mod zeta;

/// Centralized tests for all algorithms
#[cfg(test)]
mod tests;

/// Common trait for all primality testing algorithms
///
/// Implement this trait to add your own primality testing algorithm
/// to the Erato library. Your algorithm will automatically integrate
/// with the registry and benchmarking system.
pub trait PrimalityTest<N: PrimInt> {
    /// Returns the name of the algorithm
    ///
    /// This name is used for identification in the registry and benchmarks.
    /// Should be descriptive and unique.
    fn name(&self) -> &'static str;

    /// Tests whether the given number is prime
    ///
    /// # Arguments
    ///
    /// * `n` - The integer to test for primality
    ///
    /// # Returns
    ///
    /// `true` if n is (very likely to be) prime, `false` if n is definitely composite
    ///
    /// # Note
    ///
    /// For deterministic algorithms, this always returns the correct result.
    /// For probabilistic algorithms, false negatives (saying a prime is composite)
    /// are impossible, but false positives are extremely unlikely with good witnesses.
    fn is_prime(&self, n: N) -> bool;
}

/// Registry for managing and comparing primality testing algorithms
///
/// The registry maintains a collection of algorithm implementations and provides
/// convenient methods for registering new algorithms and accessing them by name.
pub struct PrimalityRegistry<N: PrimInt> {
    algorithms: Vec<Box<dyn PrimalityTest<N>>>,
}

impl<N: PrimInt + ToPrimitive + FromPrimitive + Unsigned> PrimalityRegistry<N> {
    /// Creates a new empty registry
    ///
    /// Use this if you want to manually register specific algorithms.
    /// To get all registered algorithms at once, use `with_all_algorithms()`.
    pub fn new() -> Self {
        PrimalityRegistry {
            algorithms: Vec::new(),
        }
    }

    /// Creates a registry with all available algorithms
    ///
    /// This is the recommended way to create a registry if you want
    /// to use all implemented algorithms.
    pub fn with_all_algorithms() -> Self {
        let mut registry = PrimalityRegistry::new();

        // Register all algorithms here - add new ones as you create them
        registry.register(sieve::SieveAlgorithm::default());
        registry.register(miller_rabin::MillerRabinAlgorithm::default());
        registry.register(zeta::ZetaAlgorithm::default());

        registry
    }

    /// Registers a new primality test algorithm
    ///
    /// # Arguments
    ///
    /// * `algo` - The algorithm to register, must implement `PrimalityTest`
    pub fn register<T: PrimalityTest<N> + 'static>(&mut self, algo: T) {
        self.algorithms.push(Box::new(algo));
    }

    /// Returns a slice of all registered algorithms
    pub fn algorithms(&self) -> &[Box<dyn PrimalityTest<N>>] {
        &self.algorithms
    }

    /// Finds an algorithm by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the algorithm to find
    ///
    /// # Returns
    ///
    /// Some reference to the algorithm if found, None otherwise
    pub fn get_by_name(&self, name: &str) -> Option<&Box<dyn PrimalityTest<N>>> {
        self.algorithms.iter().find(|a| a.name() == name)
    }
}

impl<N: PrimInt + ToPrimitive + FromPrimitive + Unsigned> Default for PrimalityRegistry<N> {
    fn default() -> Self {
        Self::new()
    }
}
