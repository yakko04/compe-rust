use proconio::input;

fn gcd(a: usize, b:usize) -> usize{
    if b == 0 {a} else {gcd(b,a % b)}
}
fn main() {
    input!{
        a: usize,
        b: usize,
    }

    let gc = gcd(a,b);
    let inf = 1e18 as usize;
    if a / gc > inf / b {
        println!("Large");
    } else {
        println!("{}", a / gc * b);
    }

}