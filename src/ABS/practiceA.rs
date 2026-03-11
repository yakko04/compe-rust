use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        s: String,
    }

    let ans: i32 = a + b + c;
    println!("{} {}", ans, s)
}
