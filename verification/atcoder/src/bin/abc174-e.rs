#[macro_use]
extern crate input as _;

use partition_point::RangeBoundsExt as _;

fn main() {
    input! {
        n: usize,
        k: u64,
        r#as: [u64; n],
    }
    let ans = (1..1_000_000_000)
        .partition_point(|x| r#as.iter().map(|a| (a + x - 1) / x - 1).sum::<u64>() > k);
    println!("{}", ans);
}
