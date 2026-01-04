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
    primes: Vec<u8>,
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
        let num_bytes = (limit + 8) / 8;
        let mut primes = vec![0xFF; num_bytes];

        // Define a standard function instead of a closure.
        // This avoids holding a permanent mutable borrow on 'primes'.
        fn mark_composite(primes: &mut [u8], n: usize) {
            let byte_index = n / 8;
            let bit_index = n % 8;
            primes[byte_index] &= !(1 << bit_index);
        }

        // 1. Unconditionally mark 0 as composite (it's never prime).
        mark_composite(&mut primes, 0);

        // 2. Mark 1 as composite (only if it exists within our limit).
        if limit >= 1 {
            mark_composite(&mut primes, 1);
        }

        let sqrt_limit = (limit as f64).sqrt() as usize;

        for num in 2..=sqrt_limit {
            // 1. Immutable Borrow: We read from primes here.
            // This is safe because 'mark_composite' isn't holding onto primes anymore.
            let byte_index = num / 8;
            let bit_index = num % 8;
            let is_prime = (primes[byte_index] & (1 << bit_index)) != 0;

            if is_prime {
                let start_index = num.saturating_mul(num);
                if start_index > limit {
                    break;
                }

                for multiple in (start_index..=limit).step_by(num) {
                    // 2. Mutable Borrow: We write to primes here.
                    // The borrow starts and ends strictly within this function call.
                    mark_composite(&mut primes, multiple);
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
            return false;
        }
        let byte_index = n / 8;
        let bit_index = n % 8;

        // Check if the specific bit is set to 1
        (self.primes[byte_index] & (1 << bit_index)) != 0
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
        (0..=self.limit).filter(move |&n| self.is_prime(n))
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
