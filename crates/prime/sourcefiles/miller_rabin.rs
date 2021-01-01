#[allow(clippy::clippy::many_single_char_names)]
pub fn is_prime(n: u64) -> bool {
    // https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test

    if n == 2 {
        return true;
    }
    if n == 0 || n == 1 || n % 2 == 0 {
        return false;
    }

    let d = (n - 1) >> (n - 1).trailing_zeros();
    let r = (n - 1).trailing_zeros();

    let witnesses = if n < 2_047 {
        &[2][..]
    } else if n < 1_373_653 {
        &[2, 3][..]
    } else if n < 9_080_191 {
        &[31, 73][..]
    } else if n < 25_326_001 {
        &[2, 3, 5][..]
    } else if n < 3_215_031_751 {
        &[2, 3, 5, 7][..]
    } else if n < 4_759_123_141 {
        &[2, 7, 61][..]
    } else if n < 1_112_004_669_633 {
        &[2, 13, 23, 1_662_803][..]
    } else if n < 2_152_302_898_747 {
        &[2, 3, 5, 7, 11][..]
    } else if n < 3_474_749_660_383 {
        &[2, 3, 5, 7, 11, 13][..]
    } else if n < 341_550_071_728_321 {
        &[2, 3, 5, 7, 11, 13, 17][..]
    } else {
        &[2, 3, 5, 7, 11, 13, 17, 19, 23][..]
    };

    'witness_loop: for &a in witnesses {
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'witness_loop;
        }
        for _ in 0..r - 1 {
            x = mod_mul(x, x, n);
            if x == 1 {
                return false;
            }
            if x == n - 1 {
                continue 'witness_loop;
            }
        }
        return false;
    }

    true
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut ret = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            ret = mod_mul(ret, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp /= 2;
    }
    ret
}

fn mod_mul(lhs: u64, rhs: u64, modulus: u64) -> u64 {
    if let Some(mul) = lhs.checked_mul(rhs) {
        mul % modulus
    } else {
        (u128::from(lhs) * u128::from(rhs) % u128::from(modulus)) as _
    }
}

#[cfg(test)]
mod tests {
    use primal_sieve::Sieve;

    #[test]
    fn test() {
        const LIMIT: usize = 1_000_000;
        let sieve = Sieve::new(LIMIT);
        for x in 0..LIMIT {
            assert_eq!(sieve.is_prime(x), super::is_prime(x as _));
        }
    }
}
