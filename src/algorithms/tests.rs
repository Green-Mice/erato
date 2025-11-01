#[cfg(test)]
mod algorithm_tests {
    use crate::{PrimalityRegistry, PrimalityTest};

    /// Test suite that runs on all registered algorithms
    fn test_all_algorithms<F>(test_fn: F, test_name: &str)
    where
        F: Fn(&dyn PrimalityTest<u64>) -> bool,
    {
        let registry = PrimalityRegistry::<u64>::with_all_algorithms();
        
        for algo in registry.algorithms() {
            let result = test_fn(algo.as_ref());
            assert!(
                result,
                "Algorithm '{}' failed test: {}",
                algo.name(),
                test_name
            );
        }
    }

    #[test]
    fn test_edge_case_zero() {
        test_all_algorithms(
            |algo| !algo.is_prime(0),
            "0 should not be prime"
        );
    }

    #[test]
    fn test_edge_case_one() {
        test_all_algorithms(
            |algo| !algo.is_prime(1),
            "1 should not be prime"
        );
    }

    #[test]
    fn test_edge_case_two() {
        test_all_algorithms(
            |algo| algo.is_prime(2),
            "2 should be prime"
        );
    }

    #[test]
    fn test_edge_case_three() {
        test_all_algorithms(
            |algo| algo.is_prime(3),
            "3 should be prime"
        );
    }

    #[test]
    fn test_small_primes() {
        let small_primes = vec![5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        
        test_all_algorithms(
            |algo| {
                small_primes.iter().all(|&n| algo.is_prime(n))
            },
            "all small primes should be detected as prime"
        );
    }

    #[test]
    fn test_small_composites() {
        let small_composites = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28, 30, 32];
        
        test_all_algorithms(
            |algo| {
                small_composites.iter().all(|&n| !algo.is_prime(n))
            },
            "all small composites should be detected as composite"
        );
    }

    #[test]
    fn test_medium_primes() {
        let medium_primes = vec![101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197];
        
        test_all_algorithms(
            |algo| {
                medium_primes.iter().all(|&n| algo.is_prime(n))
            },
            "all medium primes should be detected as prime"
        );
    }

    #[test]
    fn test_medium_composites() {
        let medium_composites = vec![100, 102, 104, 105, 106, 108, 110, 111, 112, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124];
        
        test_all_algorithms(
            |algo| {
                medium_composites.iter().all(|&n| !algo.is_prime(n))
            },
            "all medium composites should be detected as composite"
        );
    }

    #[test]
    fn test_large_primes() {
        let large_primes = vec![1009, 10007, 100003, 1000003, 10000019, 100000007, 1000000007];
        
        test_all_algorithms(
            |algo| {
                large_primes.iter().all(|&n| algo.is_prime(n))
            },
            "all large primes should be detected as prime"
        );
    }

    #[test]
    fn test_large_composites() {
        let large_composites = vec![1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
        
        test_all_algorithms(
            |algo| {
                large_composites.iter().all(|&n| !algo.is_prime(n))
            },
            "all large composites should be detected as composite"
        );
    }

    #[test]
    fn test_carmichael_numbers() {
        // Carmichael numbers are composite but pass Fermat's test
        let carmichael = vec![561, 1105, 1729, 2465, 2821, 6601, 8911, 10585, 15841, 29341];
        
        test_all_algorithms(
            |algo| {
                carmichael.iter().all(|&n| !algo.is_prime(n))
            },
            "Carmichael numbers should be detected as composite"
        );
    }

    #[test]
    fn test_mersenne_primes() {
        // Mersenne primes: 2^p - 1 where p is prime
        let mersenne_primes = vec![3, 7, 31, 127, 8191, 131071, 524287, 2147483647];
        
        test_all_algorithms(
            |algo| {
                mersenne_primes.iter().all(|&n| algo.is_prime(n))
            },
            "Mersenne primes should be detected as prime"
        );
    }

    #[test]
    fn test_fermat_primes() {
        // Fermat primes: 2^(2^n) + 1
        let fermat_primes = vec![3, 5, 17, 257, 65537];
        
        test_all_algorithms(
            |algo| {
                fermat_primes.iter().all(|&n| algo.is_prime(n))
            },
            "Fermat primes should be detected as prime"
        );
    }

    #[test]
    fn test_powers_of_two() {
        let powers_of_two = vec![4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];
        
        test_all_algorithms(
            |algo| {
                powers_of_two.iter().all(|&n| !algo.is_prime(n))
            },
            "powers of 2 (except 2) should be composite"
        );
    }

    #[test]
    fn test_perfect_squares() {
        let perfect_squares = vec![4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225];
        
        test_all_algorithms(
            |algo| {
                perfect_squares.iter().all(|&n| !algo.is_prime(n))
            },
            "perfect squares should be composite"
        );
    }

    #[test]
    fn test_twin_primes() {
        // Twin primes: primes that differ by 2
        let twin_primes = vec![
            (3, 5), (5, 7), (11, 13), (17, 19), (29, 31), (41, 43), 
            (59, 61), (71, 73), (101, 103), (107, 109), (137, 139),
            (149, 151), (179, 181), (191, 193), (197, 199)
        ];
        
        test_all_algorithms(
            |algo| {
                twin_primes.iter().all(|(p1, p2)| algo.is_prime(*p1) && algo.is_prime(*p2))
            },
            "twin primes should both be detected as prime"
        );
    }

    #[test]
    fn test_sophie_germain_primes() {
        // Sophie Germain primes: p is prime and 2p+1 is also prime
        let sophie_germain = vec![2, 3, 5, 11, 23, 29, 41, 53, 83, 89, 113, 131];
        
        test_all_algorithms(
            |algo| {
                sophie_germain.iter().all(|&p| {
                    algo.is_prime(p) && algo.is_prime(2 * p + 1)
                })
            },
            "Sophie Germain primes and their safe primes should be detected"
        );
    }

    #[test]
    fn test_highly_composite_numbers() {
        // Numbers with many factors
        let highly_composite = vec![12, 24, 36, 48, 60, 120, 180, 240, 360, 720, 840, 1260, 1680];
        
        test_all_algorithms(
            |algo| {
                highly_composite.iter().all(|&n| !algo.is_prime(n))
            },
            "highly composite numbers should be detected as composite"
        );
    }

    #[test]
    fn test_even_numbers_except_two() {
        let even_numbers = vec![4, 6, 8, 10, 12, 14, 16, 18, 20, 100, 1000, 10000, 100000];
        
        test_all_algorithms(
            |algo| {
                even_numbers.iter().all(|&n| !algo.is_prime(n))
            },
            "even numbers (except 2) should be composite"
        );
    }

    #[test]
    fn test_primes_ending_in_1() {
        let primes_ending_1 = vec![11, 31, 41, 61, 71, 101, 131, 151, 181, 191];
        
        test_all_algorithms(
            |algo| {
                primes_ending_1.iter().all(|&n| algo.is_prime(n))
            },
            "primes ending in 1 should be detected as prime"
        );
    }

    #[test]
    fn test_primes_ending_in_3() {
        let primes_ending_3 = vec![3, 13, 23, 43, 53, 73, 83, 103, 113, 163, 173, 193];
        
        test_all_algorithms(
            |algo| {
                primes_ending_3.iter().all(|&n| algo.is_prime(n))
            },
            "primes ending in 3 should be detected as prime"
        );
    }

    #[test]
    fn test_primes_ending_in_7() {
        let primes_ending_7 = vec![7, 17, 37, 47, 67, 97, 107, 127, 137, 157, 167, 197];
        
        test_all_algorithms(
            |algo| {
                primes_ending_7.iter().all(|&n| algo.is_prime(n))
            },
            "primes ending in 7 should be detected as prime"
        );
    }

    #[test]
    fn test_primes_ending_in_9() {
        let primes_ending_9 = vec![19, 29, 59, 79, 89, 109, 139, 149, 179, 199];
        
        test_all_algorithms(
            |algo| {
                primes_ending_9.iter().all(|&n| algo.is_prime(n))
            },
            "primes ending in 9 should be detected as prime"
        );
    }

    #[test]
    fn test_semiprimes() {
        // Products of exactly two primes
        let semiprimes = vec![
            4,      // 2 * 2
            6,      // 2 * 3
            9,      // 3 * 3
            10,     // 2 * 5
            14,     // 2 * 7
            15,     // 3 * 5
            21,     // 3 * 7
            22,     // 2 * 11
            25,     // 5 * 5
            26,     // 2 * 13
        ];
        
        test_all_algorithms(
            |algo| {
                semiprimes.iter().all(|&n| !algo.is_prime(n))
            },
            "semiprimes should be detected as composite"
        );
    }

    #[test]
    fn test_very_large_primes() {
        let very_large_primes = vec![
            10_000_000_019,
            100_000_000_003,
            1_000_000_000_039,
        ];
        
        test_all_algorithms(
            |algo| {
                very_large_primes.iter().all(|&n| algo.is_prime(n))
            },
            "very large primes should be detected as prime"
        );
    }

    #[test]
    fn test_consistency_across_algorithms() {
        let registry = PrimalityRegistry::<u64>::with_all_algorithms();
        let test_numbers = vec![
            0, 1, 2, 3, 4, 5, 17, 100, 561, 1009, 10007, 100003, 1000003
        ];
        
        // Get reference results from first algorithm
        let reference_algo = &registry.algorithms()[0];
        let reference_results: Vec<bool> = test_numbers
            .iter()
            .map(|&n| reference_algo.is_prime(n))
            .collect();
        
        // Check all other algorithms agree
        for algo in registry.algorithms().iter().skip(1) {
            for (i, &n) in test_numbers.iter().enumerate() {
                let result = algo.is_prime(n);
                assert_eq!(
                    result, reference_results[i],
                    "Algorithm '{}' disagrees with '{}' on number {}",
                    algo.name(), reference_algo.name(), n
                );
            }
        }
    }

    #[test]
    fn test_all_algorithms_registered() {
        let registry = PrimalityRegistry::<u64>::with_all_algorithms();
        let algo_count = registry.algorithms().len();
        
        assert!(
            algo_count >= 3,
            "Expected at least 3 algorithms (Sieve, Miller-Rabin, Riemann), found {}",
            algo_count
        );
        
        // Check that known algorithms are present
        assert!(
            registry.get_by_name("Sieve of Eratosthenes").is_some(),
            "Sieve of Eratosthenes should be registered"
        );
        assert!(
            registry.get_by_name("Miller-Rabin").is_some(),
            "Miller-Rabin should be registered"
        );
        assert!(
            registry.get_by_name("Riemann Zeta").is_some(),
            "Riemann Zeta Hypothesis should be registered"
        );
    }

    #[test]
    fn test_algorithm_names_unique() {
        let registry = PrimalityRegistry::<u64>::with_all_algorithms();
        let mut names: Vec<&str> = registry.algorithms()
            .iter()
            .map(|a| a.name())
            .collect();
        
        let original_len = names.len();
        names.sort();
        names.dedup();
        
        assert_eq!(
            names.len(), original_len,
            "Algorithm names should be unique"
        );
    }
}
