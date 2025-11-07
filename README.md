# Erato

A Rust library for primality testing algorithms with integrated benchmarking using Criterion.

## Features

- **Multiple primality testing algorithms**
  - Sieve of Eratosthenes (trial division)
  - Miller-Rabin (probabilistic, deterministic)
  - Riemann Zeta (spectroscopic analysis using zeta function zeros)
- **Extensible architecture** - Easily add new algorithms
- **Integrated benchmarking** - Compare algorithm performance automatically
- **Zero-cost abstractions** - Efficient implementations
- **Well-tested** - Comprehensive test suite

## Quick Start

### Using individual functions

```rust
use erato::{is_prime_sieve, is_prime_miller_rabin};

fn main() {
    println!("17 is prime: {}", is_prime_sieve(17));
    println!("100 is prime: {}", is_prime_miller_rabin(100, 20));
}
```

### Using the trait-based interface

```rust
use erato::{PrimalityTest, SieveAlgorithm, MillerRabinAlgorithm};

fn main() {
    let sieve = SieveAlgorithm;
    let miller_rabin = MillerRabinAlgorithm;
    
    println!("Sieve: {}", sieve.is_prime(17));
    println!("Miller-Rabin: {}", miller_rabin.is_prime(17));
}
```

### Using the registry

```rust
use erato::PrimalityRegistry;

fn main() {
    let registry = PrimalityRegistry::with_all_algorithms();
    
    for algo in registry.algorithms() {
        println!("{}: {}", algo.name(), algo.is_prime(17));
    }
}
```

## Benchmarking

Run the benchmarks to compare algorithm performance:

```bash
cargo bench
```

This generates HTML reports in `target/criterion/` with detailed performance analysis.

Benchmark categories:
- Small primes (< 1,000)
- Small composites
- Medium primes (10,000 - 100,000)
- Medium composites
- Large primes (1,000,000 - 1,000,000,000)
- Very large primes (up to 100 billion)

## Adding a New Algorithm

1. Create a new file in `src/algorithms/`:

```rust
use crate::PrimalityTest;

#[derive(Default)]
pub struct MyAlgorithm;

impl PrimalityTest for MyAlgorithm {
    fn name(&self) -> &'static str {
        "My Algorithm"
    }

    fn is_prime(&self, n: u64) -> bool {
        // Your implementation
    }
}

pub fn is_prime_my_algorithm(n: u64) -> bool {
    is_prime_my_algorithm_internal(n)
}

fn is_prime_my_algorithm_internal(n: u64) -> bool {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert!(is_prime_my_algorithm(17));
    }
}
```

2. Add the module to `src/algorithms/mod.rs`:

```rust
pub mod my_algorithm;
```

3. Register it in the registry (in `mod.rs`):

```rust
registry.register(my_algorithm::MyAlgorithm::default());
```

4. Export from `src/lib.rs`:

```rust
pub use algorithms::my_algorithm::{is_prime_my_algorithm, MyAlgorithm};
```

That's it! Your algorithm will automatically be included in benchmarks and the public API.

## Riemann Hypothesis-based Testing

The library includes an experimental primality test based on the **Riemann Hypothesis (RH)**, which remains one of the most important unproven conjectures in mathematics. While RH is not proven, it has been computationally verified for the first 10 trillion non-trivial zeros of the zeta function, making it highly reliable for practical applications within reasonable bounds.

This algorithm exploits the explicit formula connecting the Riemann zeta function zeros to prime distribution. It uses a **spectroscopic approach**: the first 50 non-trivial zeros create oscillatory "frequency signatures" that distinguish primes from composites through interference patterns. The method computes:
- Oscillation coherence across zeta zeros (Σ cos(γ·log(n)))
- Chebyshev psi function jumps estimated via the explicit formula
- Spectral resonance patterns characteristic of prime numbers
- Phase alignment analysis inspired by Fourier transforms

While the algorithm still performs trial division for verification, it uses RH-derived properties to compute prime probability scores that guide the search strategy. This demonstrates the deep connection between analytic number theory and computational primality testing, offering an educational glimpse into how the zeros of zeta encode information about primes.

**Note**: This implementation assumes RH for its analytical optimizations. Since RH is verified computationally to very high limits, the algorithm is reliable for all practical integer ranges, though it remains theoretical for arbitrarily large numbers.

### Usage
```rust
use erato::{is_prime_zeta, ZetaAlgorithm, PrimalityTest};

fn main() {
    // Direct function call
    println!("17 is prime: {}", is_prime_zeta(17));
    
    // Using the trait interface
    let zeta = ZetaAlgorithm;
    println!("Using {}: {}", zeta.name(), zeta.is_prime(1000000007));
}
```

## Zeta Example

```bash
cargo run --example zeta_primality
```
## WebAssembly Demo

This project includes a web interface that uses the Zeta primality algorithm compiled to WebAssembly for maximum performance.

### Prerequisites

Install `wasm-pack`:

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Building

Compile the Rust code to WebAssembly:

```bash
wasm-pack build --target web --release
```

This generates the `pkg/` directory containing:
- `erato.js` - JavaScript bindings
- `erato_bg.wasm` - Compiled WebAssembly module

### Running

Serve the files with any HTTP server:

```bash
python3 -m http.server 8080
```

Then open `http://localhost:8080` in your browser.

The `index.html` automatically loads the WASM module:

```javascript
const module = await import('./pkg/erato.js');
await module.default();
```

![WebAssembly Demo](https://github.com/Green-Mice/erato/sample1.png)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

- [Miller-Rabin Primality Test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
- [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
- [Riemann hypothesis](https://en.wikipedia.org/wiki/Riemann_hypothesis)
- [Criterion.rs](https://bheisler.github.io/criterion.rs/book/)
