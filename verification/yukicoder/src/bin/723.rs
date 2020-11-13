// verify-helper: PROBLEM https://yukicoder.me/problems/no/723

#[macro_use]
extern crate input as _;

use partition_point::RangeBoundsExt as _;

fn main() {
    input! {
        n: usize,
        x: u32,
        mut r#as: [u32; n],
    }

    r#as.sort();

    let ans = (0..n)
        .map(|i| {
            let s = (i..n).partition_point(|j| r#as[i] + r#as[j] <= x);
            let t = (i..n).partition_point(|j| r#as[i] + r#as[j] < x);
            2 * (s - t) - usize::from(2 * r#as[i] == x)
        })
        .sum::<usize>();
    println!("{}", ans);
}
