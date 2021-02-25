#[macro_use]
extern crate input as _;

fn main() {
    input! {
        k: u64,
    }
    let ans = (1..=k)
        .flat_map(|a| (1..=k).map(move |b| (a, b)))
        .flat_map(|(a, b)| (1..=k).map(move |c| (a, b, c)))
        .map(|(a, b, c)| gcd::gcd(gcd::gcd(a, b), c))
        .sum::<u64>();
    println!("{}", ans);
}
