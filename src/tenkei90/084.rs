use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let total = n * (n - 1) / 2;
    let mut sub = 0;
    let mut cnt = 1usize;
    let chars: Vec<u8> = s.bytes().collect();
    for i in 1..n {
        if chars[i] == chars[i - 1] {
            cnt += 1;
        } else {
            sub += cnt * (cnt - 1) / 2;
            cnt = 1;
        }
    }
    sub += cnt * (cnt - 1) / 2;
    println!("{}", total - sub);
}
