use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }
    let mut count = 0u32;
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            count += 1;
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        count += 1;
    }
    let ans = if count <= 1 { 0 } else { (count as f64).log2().ceil() as u32 };
    println!("{}", ans);
}
