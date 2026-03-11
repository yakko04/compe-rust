use proconio::input;

fn main() {
    input!{
        n:u32,
        a:u32,
        b:u32,
    }
    let mut ans = 0;

    for i in 1..=n {
        let sum: u32 = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
        if sum >= a && sum <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}