use std::ops::{Add, Mul, Sub};

pub fn montmort<T: From<usize> + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>>(
    max: usize,
) -> Vec<T> {
    let mut montmort = vec![T::from(0); max + 1];
    for k in 2..=max {
        montmort[k] = montmort[k - 1] * T::from(k);
        if k & 1 == 1 {
            montmort[k] = montmort[k] - T::from(1);
        } else {
            montmort[k] = montmort[k] + T::from(1);
        }
    }
    montmort
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test() {
        assert_eq!(
            vec![0usize, 0, 1, 2, 9, 44, 265, 1854, 14833, 133_496, 1_334_961],
            super::montmort(10),
        );
    }
}
