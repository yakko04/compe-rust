use proconio::input;

fn main() {
    input!{
        n:usize,
        y:usize,
    }

    let mut ans = (-1, -1, -1);

    for i in 0..=n {
        for j in 0..=(n-i) {
            if i * 10000 + j * 5000 + (n - i - j) * 1000 == y {
                ans = (i as i32, j as i32, (n-i-j) as i32);
                break
            }
        }
    }

    println!("{} {} {}", ans.0, ans.1, ans.2);
}