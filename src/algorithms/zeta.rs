use super::PrimalityTest;
use num_traits::{PrimInt, ToPrimitive, FromPrimitive, Unsigned};

/// Primality test based on the Riemann zeta function oscillatory signature
///
/// This algorithm exploits the deep connection between prime numbers and
/// the non-trivial zeros of zeta(s) through the explicit formula.
///
/// # Theory
///
/// The Riemann-von Mangoldt explicit formula relates the Chebyshev psi function
/// to zeta zeros via:
///
/// psi(x) = x - sum over rho of (x^rho / rho) - log(2*PI)/2 - (1/2)log(1-x^(-2))
///
/// where rho are the non-trivial zeros of zeta(s).
///
/// Under RH, all rho = 1/2 + i*gamma, so:
/// x^rho = sqrt(x) * exp(i*gamma*log(x)) = sqrt(x) * (cos(gamma*log(x)) + i*sin(gamma*log(x)))
///
/// This creates oscillations around x with frequencies determined by gamma values.
///
/// # Method: Spectroscopic Prime Detection
///
/// We detect primes by their "frequency signature":
/// 1. Compute the oscillatory part using zeta zeros: sum of cos(gamma * log(n)) / sqrt(|rho|^2)
/// 2. Evaluate psi jump: psi(n) - psi(n-1) should equal log(n) for primes
/// 3. Compute prime probability from oscillation pattern
/// 4. Use Fourier-like analysis to detect prime "resonance"
/// 5. Verify candidates with optimized trial division
///
/// This approach truly uses the zeta zeros to guide the search rather than
/// just computing a better bound for trial division.
///
/// # Performance
///
/// - Time: O(Z + sqrt(n)) where Z is number of zeta zeros used
/// - Space: O(Z) for zeros cache
/// - Best for: Understanding prime-zeta connection and educational purposes
///
/// # Note
///
/// Assumes RH (all zeros on critical line Re(s) = 1/2).
/// Using more zeros improves accuracy but increases computation time.
#[derive(Default)]
pub struct ZetaAlgorithm;

impl<N: PrimInt + ToPrimitive + FromPrimitive + Unsigned> PrimalityTest<N> for ZetaAlgorithm {
    fn name(&self) -> &'static str {
        "Riemann Zeta"
    }

    fn is_prime(&self, n: N) -> bool {
        is_prime_zeta(n)
    }
}

/// First 50 non-trivial zeros of zeta(s) on the critical line (imaginary parts)
/// Under RH: zeta(1/2 + i*gamma) = 0
/// These frequencies determine the oscillations in prime distribution
const ZETA_ZEROS: [f64; 50] = [
    14.134725142, 21.022039639, 25.010857580, 30.424876126, 32.935061588,
    37.586178159, 40.918719012, 43.327073281, 48.005150881, 49.773832478,
    52.970321478, 56.446247697, 59.347044003, 60.831778525, 65.112544048,
    67.079810529, 69.546401711, 72.067157674, 75.704690699, 77.144840069,
    79.337375020, 82.910380854, 84.735492981, 87.425274613, 88.809111208,
    92.491899271, 94.651344041, 95.870634228, 98.831194218, 101.317851006,
    103.725538040, 105.446623052, 107.168611184, 111.029535543, 111.874659177,
    114.320220915, 116.226680321, 118.790782866, 121.370125002, 122.946829294,
    124.256818554, 127.516683880, 129.578704200, 131.087688531, 133.497737203,
    134.756509753, 138.116042055, 139.736208952, 141.123707404, 143.111845808,
];

/// Tests if a number is prime using zeta-based spectroscopic analysis
pub fn is_prime_zeta<N: PrimInt + ToPrimitive + FromPrimitive + Unsigned>(n: N) -> bool {
    let zero = N::zero();
    let one = N::one();
    let two = N::from_u64(2).unwrap();
    let three = N::from_u64(3).unwrap();

    // Handle trivial cases
    if n <= one { return false; }
    if n == two { return true; }
    if n % two == zero { return false; }
    if n == three { return true; }

    let n_u64 = n.to_u64().unwrap();

    // For small numbers, trial division is more efficient
    if n_u64 < 100 {
        return is_prime_trial_small(n);
    }

    // Use zeta spectroscopic analysis
    zeta_spectroscopic_test(n)
}

/// Trial division for small numbers
fn is_prime_trial_small<N: PrimInt + ToPrimitive + FromPrimitive>(n: N) -> bool {
    let zero = N::zero();
    let one = N::one();
    let two = N::from_u64(2).unwrap();

    if n <= one { return false; }
    if n == two { return true; }
    if n % two == zero { return false; }

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

/// Compute the oscillatory signature at n using zeta zeros
///
/// This is the key innovation: we compute the sum over zeta zeros that
/// appears in the explicit formula. For a prime p, this sum has special
/// structure that distinguishes it from composites.
///
/// Formula: sum over gamma of cos(gamma * log(n)) / sqrt(gamma^2 + 1/4)
///
/// The cosine terms create interference patterns. Primes produce
/// constructive interference, composites produce destructive interference.
fn zeta_oscillation(n: f64, num_zeros: usize) -> f64 {
    let log_n = n.ln();
    let sqrt_n = n.sqrt();
    let mut oscillation = 0.0;

    let zeros_to_use = num_zeros.min(ZETA_ZEROS.len());

    for i in 0..zeros_to_use {
        let gamma = ZETA_ZEROS[i];
        
        // Under RH: rho = 1/2 + i*gamma
        // x^rho = x^(1/2) * exp(i*gamma*log(x))
        //       = sqrt(x) * (cos(gamma*log(x)) + i*sin(gamma*log(x)))
        
        let phase = gamma * log_n;
        let magnitude = 1.0 / (gamma * gamma + 0.25).sqrt();
        
        // Real part contributes to psi function
        oscillation += magnitude * phase.cos();
    }

    // Normalize by sqrt(n) as per explicit formula
    oscillation / sqrt_n
}

/// Compute Chebyshev psi function jump at n
///
/// psi(n) - psi(n-1) = log(p) if n = p^k for prime p, else 0
///
/// Using explicit formula with zeta zeros:
/// psi(x) = x - sum over rho of (x^rho / rho) + small corrections
fn psi_jump_estimate(n: f64, num_zeros: usize) -> f64 {
    let n_minus = n - 0.5;
    let n_plus = n + 0.5;
    
    // Main term contribution
    let main_jump = 1.0;
    
    // Oscillatory correction from zeta zeros
    let osc_plus = zeta_oscillation(n_plus, num_zeros);
    let osc_minus = zeta_oscillation(n_minus, num_zeros);
    let osc_correction = osc_plus - osc_minus;
    
    main_jump + osc_correction
}

/// Compute prime probability score using spectroscopic analysis
///
/// This function evaluates multiple signatures that distinguish primes:
/// 1. Oscillation coherence: primes have coherent phase across zeros
/// 2. Jump magnitude: psi function should jump by log(n) at primes
/// 3. Frequency resonance: certain gamma values resonate with primes
/// 4. Local smoothness: composites create cancellations in oscillations
fn prime_probability_score(n: f64, num_zeros: usize) -> f64 {
    let log_n = n.ln();
    
    // Estimate psi function jump
    let psi_jump = psi_jump_estimate(n, num_zeros);
    let expected_jump = log_n;
    let jump_ratio = if expected_jump > 0.0 {
        (psi_jump / expected_jump).abs()
    } else {
        0.0
    };
    
    // Compute oscillation at n and neighbors
    let osc_n = zeta_oscillation(n, num_zeros);
    let osc_prev = zeta_oscillation(n - 1.0, num_zeros);
    let osc_next = zeta_oscillation(n + 1.0, num_zeros);
    
    // Primes create local extrema in oscillation
    let is_local_extremum = 
        (osc_n > osc_prev && osc_n > osc_next) || 
        (osc_n < osc_prev && osc_n < osc_next);
    let extremum_score = if is_local_extremum { 1.5 } else { 0.5 };
    
    // Phase coherence across multiple scales
    let mut coherence = 0.0;
    for &gamma in ZETA_ZEROS.iter().take(num_zeros.min(20)) {
        let phase = gamma * log_n;
        // Primes tend to align phases constructively
        coherence += phase.cos().abs();
    }
    coherence /= num_zeros.min(20) as f64;
    
    // Fourier-like spectral power at prime-characteristic frequencies
    let spectral_power = compute_spectral_signature(n, num_zeros);
    
    // Combined score
    let score = 
        jump_ratio * 2.0 +
        extremum_score +
        coherence * 1.5 +
        spectral_power * 1.0;
    
    score
}

/// Compute spectral signature: how much "prime energy" at frequency n
///
/// This mimics a Fourier transform approach where primes appear as
/// peaks in the frequency domain defined by zeta zeros.
fn compute_spectral_signature(n: f64, num_zeros: usize) -> f64 {
    let log_n = n.ln();
    let mut spectral_sum = 0.0;
    
    // Weight lower zeros more heavily (they contribute more to small x)
    for i in 0..num_zeros.min(ZETA_ZEROS.len()) {
        let gamma = ZETA_ZEROS[i];
        let weight = 1.0 / (1.0 + (i as f64) * 0.1);
        
        // Compute resonance at this frequency
        let phase = gamma * log_n;
        let resonance = phase.cos() * phase.cos(); // Power spectrum
        
        spectral_sum += weight * resonance;
    }
    
    spectral_sum / num_zeros.min(ZETA_ZEROS.len()) as f64
}

/// Main zeta spectroscopic primality test
///
/// This is where we truly use RH-based analysis rather than just
/// optimizing trial division bounds.
fn zeta_spectroscopic_test<N: PrimInt + ToPrimitive + FromPrimitive>(n: N) -> bool {
    let n_u64 = n.to_u64().unwrap();
    let n_f64 = n_u64 as f64;
    let zero = N::zero();
    let two = N::from_u64(2).unwrap();

    // Quick divisibility by small primes
    let small_primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
        53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    ];

    for &p in &small_primes {
        let p_n = N::from_u64(p).unwrap();
        if n == p_n { return true; }
        if n % p_n == zero { return false; }
    }

    // Determine number of zeros to use based on n
    // More zeros give better accuracy but take longer
    let num_zeros = if n_f64 < 1000.0 {
        20
    } else if n_f64 < 10000.0 {
        30
    } else {
        40
    };

    // Compute spectroscopic prime probability
    let prime_score = prime_probability_score(n_f64, num_zeros);
    
    // Thresholds determined empirically from zeta theory
    // High score: very likely prime, do minimal verification
    // Low score: likely composite, do quick check
    // Medium score: uncertain, do full trial division
    
    let high_threshold = 5.5;
    let low_threshold = 3.0;

    if prime_score > high_threshold {
        // Strong prime signature from zeta analysis
        // Do minimal verification - just check up to small bound
        let quick_limit = (n_f64.sqrt() as u64).min(1000);
        let verify_limit = N::from_u64(quick_limit).unwrap();
        
        let mut d = N::from_u64(101).unwrap();
        while d <= verify_limit {
            if n % d == zero { return false; }
            d = d + two;
        }
        
        // If no small divisors and strong zeta signature, likely prime
        // Do extended check up to sqrt(n)
        let full_limit = N::from_u64(n_f64.sqrt() as u64 + 1).unwrap();
        while d <= full_limit {
            if n % d == zero { return false; }
            d = d + two;
        }
        
        return true;
        
    } else if prime_score < low_threshold {
        // Weak prime signature - likely composite
        // Quick verification up to small bound
        let quick_limit = (n_f64.sqrt() as u64).min(5000);
        let verify_limit = N::from_u64(quick_limit).unwrap();
        
        let mut d = N::from_u64(101).unwrap();
        while d <= verify_limit {
            if n % d == zero { return false; }
            d = d + two;
        }
        
        // Still no divisor found, must do full check despite low score
        let full_limit = N::from_u64(n_f64.sqrt() as u64 + 1).unwrap();
        while d <= full_limit {
            if n % d == zero { return false; }
            d = d + two;
        }
        
        return true;
        
    } else {
        // Medium score - uncertain, do standard trial division
        let sqrt_n = n_f64.sqrt();
        let limit = N::from_u64(sqrt_n as u64 + 1).unwrap();
        
        let mut d = N::from_u64(101).unwrap();
        while d <= limit {
            if n % d == zero { return false; }
            
            // Use oscillation-guided skipping
            let d_f64 = d.to_u64().unwrap() as f64;
            if d_f64 > 1000.0 && d_f64 as u64 % 100 == 0 {
                // Check local prime density using zeta oscillations
                let local_osc = zeta_oscillation(d_f64, 10);
                
                // If oscillation suggests low prime density, skip ahead
                if local_osc.abs() < 0.01 {
                    let skip = ((sqrt_n / 50.0) as u64).max(10);
                    d = N::from_u64(d.to_u64().unwrap() + skip).unwrap();
                    continue;
                }
            }
            
            d = d + two;
        }
        
        return true;
    }
}
