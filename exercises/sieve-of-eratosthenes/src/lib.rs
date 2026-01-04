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
    words: Vec<usize>,
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
        // Dynamic constant: 64 on your Mac, 32 on older systems
        let bits = usize::BITS as usize;

        let num_words = limit / bits + 1;

        // usize::MAX is all 1s (whether that's 32 or 64 ones)
        let mut words = vec![usize::MAX; num_words];

        fn mark_composite(words: &mut [usize], n: usize) {
            let bits = usize::BITS as usize;
            let word_index = n / bits;
            let bit_index = n % bits;
            words[word_index] &= !(1 << bit_index);
        }

        mark_composite(&mut words, 0);
        if limit >= 1 {
            mark_composite(&mut words, 1);
        }

        let sqrt_limit = (limit as f64).sqrt() as usize;

        for num in 2..=sqrt_limit {
            let bits = usize::BITS as usize;
            let is_prime = (words[num / bits] & (1 << (num % bits))) != 0;

            if is_prime {
                let start_index = num.saturating_mul(num);
                if start_index > limit {
                    break;
                }

                for multiple in (start_index..=limit).step_by(num) {
                    // We can inline the unsafe optimization here too if we want
                    mark_composite(&mut words, multiple);
                }
            }
        }

        SieveOfEratosthenes { limit, words }
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
        let bits = usize::BITS as usize;
        (self.words[n / bits] & (1 << (n % bits))) != 0
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
        assert!(!sieve.is_prime(0));

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
