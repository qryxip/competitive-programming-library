use std::cmp;

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
    let g = |x: u64| (x * x + 1) % n;

    let mut x = 2;
    let mut y = 2;
    let mut d = 1;

    while d == 1 {
        x = g(x);
        y = g(g(y));
        d = gcd(cmp::max(x, y) - cmp::min(x, y), n);
    }

    if d == n {
        panic!("failed");
    }
    d
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
