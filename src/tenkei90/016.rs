use proconio::input;

fn main() {
    input!{
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let mut ans = 1000000001;

    for i in 0..10000 {
        for j in 0..10000 {
            if (a * i + b * j) <= n && (n - a * i - b * j) % c == 0 {
                ans = ans.min(i + j + (n - a * i - b * j) / c);
            }
        }
    }
    println!("{}", ans);
}