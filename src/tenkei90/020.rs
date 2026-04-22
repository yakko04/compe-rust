use proconio::input;

fn main() {
    input!{
        a: u128,
        b: u128,
        c: u128,
    }

    if a < c.pow(b.try_into().unwrap()) {
        println!("Yes");
    } else {
        println!("No")
    }
}