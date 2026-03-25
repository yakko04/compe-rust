use proconio::input;
fn main() {
    input!{
        a: usize,
        b: usize,
        c: usize,
    }

    fn gcd(a:usize, b:usize) -> usize {
        if a % b == 0 {
            b
        } else {
            gcd(b, a%b)
        }
    }

    let num_gcd = gcd(gcd(a,b), c);
    println!("{}", (a/num_gcd + b/num_gcd + c/num_gcd - 3));
    
}