/// A Sieve of Eratosthenes for efficiently finding prime numbers up to a specific limit.
///
/// # Examples
///
/// ```rust
/// use sieve_of_eratosthenes::SieveOfEratosthenes;
///
/// let primes = SieveOfEratosthenes::new(20);
/// assert!(primes.is_prime(19));
/// assert!(!primes.is_prime(20));
/// ```
pub struct SieveOfEratosthenes {
    limit: usize,
    // Using a simple Vec<bool> for readability.
    // For extreme memory efficiency, we would use a BitVec crate later.
    primes: Vec<bool>,
}

impl SieveOfEratosthenes {
    /// Create a new Sieve up to the given limit (inclusive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sieve_of_eratosthenes::SieveOfEratosthenes;
    /// // Find primes up to 10
    /// let s = SieveOfEratosthenes::new(10);
    ///
    /// // 2, 3, 5, 7 are primes
    /// assert_eq!(s.iter().count(), 4);
    /// ```
    pub fn new(limit: usize) -> Self {
        if limit < 2 {
            return SieveOfEratosthenes {
                limit,
                primes: vec![false; limit + 1],
            };
        }

        let mut primes = vec![true; limit + 1];
        primes[0] = false;
        primes[1] = false;

        let sqrt_limit = (limit as f64).sqrt() as usize;

        for num in 2..=sqrt_limit {
            if primes[num] {
                // Check for overflow before multiplying
                let start_index = num.saturating_mul(num);
                if start_index > limit {
                    break;
                }

                for multiple in (start_index..=limit).step_by(num) {
                    primes[multiple] = false;
                }
            }
        }

        SieveOfEratosthenes { limit, primes }
    }

    /// Check if a number is prime in O(1) time.
    ///
    /// Returns `false` if the number is greater than the limit used to create the Sieve.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sieve_of_eratosthenes::SieveOfEratosthenes;
    /// let s = SieveOfEratosthenes::new(10);
    /// assert_eq!(s.is_prime(7), true);
    /// assert_eq!(s.is_prime(4), false);
    ///
    /// // Numbers outside the limit return false
    /// assert_eq!(s.is_prime(100), false);
    /// ```
    pub fn is_prime(&self, n: usize) -> bool {
        if n > self.limit {
            // If user asks for a number outside our sieve, we can't answer
            // definitively with the cache, so we return false or panic.
            // For this simple lib, we return false.
            return false;
        }
        self.primes[n]
    }

    /// Return an iterator over the found primes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sieve_of_eratosthenes::SieveOfEratosthenes;
    /// let s = SieveOfEratosthenes::new(10);
    /// let primes: Vec<usize> = s.iter().collect();
    ///
    /// assert_eq!(primes, vec![2, 3, 5, 7]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.primes
            .iter()
            .enumerate()
            .filter_map(|(num, &is_prime)| if is_prime { Some(num) } else { None })
    }
}

// Tests to ensure it works
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_basic() {
        let sieve = SieveOfEratosthenes::new(30);
        assert!(sieve.is_prime(2));
        assert!(sieve.is_prime(3));
        assert!(sieve.is_prime(5));
        assert!(!sieve.is_prime(4));
        assert!(!sieve.is_prime(1));

        let primes: Vec<usize> = sieve.iter().collect();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_sieve_large() {
        let sieve = SieveOfEratosthenes::new(1000);
        let primes: Vec<usize> = sieve.iter().collect();
        assert_eq!(primes.len(), 168);
    }

    #[test]
    fn test_sieve_zero() {
        let sieve = SieveOfEratosthenes::new(0);
        let primes: Vec<usize> = sieve.iter().collect();
        assert_eq!(primes, vec![]);
    }

    #[test]
    fn test_sieve_one() {
        let sieve = SieveOfEratosthenes::new(1);
        let primes: Vec<usize> = sieve.iter().collect();
        assert_eq!(primes, vec![]);
    }
}
