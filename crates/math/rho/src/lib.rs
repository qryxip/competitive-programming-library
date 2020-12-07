pub fn factorize(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    }

    if miller_rabin::miller_rabin(n) {
        return vec![n];
    }

    let factor = rho(n);
    let mut ret = factorize(factor);
    ret.extend(factorize(n / factor));
    ret.sort_unstable();
    ret
}

#[allow(clippy::many_single_char_names)]
fn rho(n: u64) -> u64 {
    debug_assert!(!miller_rabin::miller_rabin(n));

    if n % 2 == 0 {
        return 2;
    }

    for cycle in 1.. {
        let g = |x: u64| (x * x + cycle) % n;
        let sub = |lhs: u64, rhs: u64| -> _ {
            if lhs < rhs {
                n + lhs - rhs
            } else {
                lhs - rhs
            }
        };
        let mut x = 2;
        let mut y = 2;
        let d = loop {
            x = g(x);
            y = g(g(y));
            let d = gcd(sub(x, y), n);
            if d > 1 {
                break d;
            }
        };
        if d < n {
            return d;
        }
    }
    unreachable!();
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        test(0, &[]);
        test(1, &[]);
        test(2, &[2]);
        test(3, &[3]);
        test(4, &[2, 2]);
        test(5, &[5]);
        test(6, &[2, 3]);
        test(7, &[7]);
        test(8, &[2, 2, 2]);
        test(9, &[3, 3]);
        test(10, &[2, 5]);

        fn test(n: u64, expected: &[u64]) {
            assert_eq!(*expected, *super::factorize(n));
        }
    }
}
