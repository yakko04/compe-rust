use ac_library::ModInt1000000007 as Mint;
use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
    }
    let ans = if n == 1 {
        Mint::new(k)
    } else if k < 3 && n >= 3 {
        Mint::new(0)
    } else {
        Mint::new(k) * Mint::new(k - 1) * Mint::new(k - 2).pow(n - 2)
    };
    println!("{}", ans);
}