use proconio::input;
use std::collections::HashSet;

fn main() {
    input!{
        n: usize,
        d: [usize; n],
    }

    let set: HashSet<_> = d.into_iter().collect();
    let ans = set.len();

    println!("{}", ans);

}