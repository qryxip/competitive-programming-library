pub fn gcd(a: u64, b: u64) -> u64 {
    if a == b {
        return a;
    }
    if a == 0 || b == 0 {
        return a | b;
    }

    let common_zeros = (a | b).trailing_zeros();
    let mut a = a >> a.trailing_zeros();
    let mut b = b >> b.trailing_zeros();

    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }
    a << common_zeros
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(2, super::gcd(10, 2));
        assert_eq!(1, super::gcd(10, 3));
        assert_eq!(14, super::gcd(56, 42));
    }
}
