// TODO:
//
// - Use better sieve altorithm
// - Use bitset

pub struct Sieve(Vec<bool>);

impl Sieve {
    pub fn new(limit: usize) -> Self {
        let mut sieve = vec![true; limit + 1];
        sieve[0] = false;
        if limit > 0 {
            sieve[1] = false;
        }
        for i in 2..=limit {
            if !sieve[i] {
                continue;
            }
            for i in (i * i..=limit).step_by(i) {
                sieve[i] = false;
            }
        }
        Self(sieve)
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.0[n]
    }

    pub fn primes<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, is_prime)| **is_prime)
            .map(|(i, _)| i)
    }

    pub fn factorize(&self, mut n: usize) -> Vec<(usize, usize)> {
        let mut factors = vec![];
        for p in self.primes() {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            if e > 0 {
                factors.push((p, e));
            }
        }
        if n > 1 {
            factors.push((n, 1));
        }
        factors
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_prime() {
        const LIMIT: usize = 10000;

        let expected = primal_sieve::Sieve::new(LIMIT);
        let expected = (0..=LIMIT)
            .map(|i| expected.is_prime(i))
            .collect::<Vec<_>>();

        let super::Sieve(actual) = super::Sieve::new(LIMIT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn primes() {
        const LIMIT: usize = 10000;

        let expected = primal_sieve::Sieve::new(LIMIT)
            .primes_from(0)
            .collect::<Vec<_>>();

        let actual = super::Sieve::new(LIMIT).primes().collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn factorize() {
        const LIMIT: usize = 1000;
        let sieve = super::Sieve::new(LIMIT);

        const N: usize = 2 * 2 * 5 * 7561;
        const EXPECTED: &[(usize, usize)] = &[(2, 2), (5, 1), (7561, 1)];
        let actual = &sieve.factorize(N)[..];
        assert_eq!(EXPECTED, actual);
    }
}
