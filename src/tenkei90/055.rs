use proconio::input;
use itertools::Itertools;

fn main() {
    input!{
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    }

    let mut result = 0;

    for v in a.iter().combinations(5) {
        if v.iter().fold(1, |acc, &&x| acc % p * (x % p)) % p == q {
            result += 1;
        }
    } 

    println!("{}", result);
}

